# BSIM4 AC NQS Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add native BSIM4 v4.8 AC charge-deficit NQS support for `ACNQSMOD=1, TRNQSMOD=0`, validated against ngspice 46 and checked against Xyce 7.10 acceptance behavior.

**Architecture:** Keep the existing QS BSIM4 DC, transient, and charge paths unchanged. Add an AC-specific BSIM4 small-signal path that applies the ngspice/Xyce NQS pole transform to gm/gmbs/gds and the intrinsic capacitance matrix using `omega * taunet`, while still using the existing overlap, junction, gate-resistance, and body-network topology. Leave transient NQS (`TRNQSMOD=1`) rejected until the later q-deficit state slice.

**Implementation note:** This slice is complete in the current worktree, and transient NQS support has also landed for the covered BSIM4 combinations. Focused verification on 2026-06-19 passed `acnqsmod1`, `rgatemod`, full `bsim4_native` (58/58), `bsim4v8` lib (35 passed, 1 ignored), `noise` lib (32/32), and the release-only full ngspice suite (113/113).

**Tech Stack:** Rust, `rspice-core`, native BSIM4 v4.8 model, ngspice 46 source/oracle runs, Xyce 7.10 open BSIM4 source and acceptance runs.

---

### Task 1: Red Oracle Test

**Files:**
- Modify: `crates/rspice-core/tests/bsim4_native.rs`

- [x] **Step 1: Add an AC NQS oracle test**

Add a test next to the existing BSIM4 AC oracle tests:

```rust
#[test]
fn acnqsmod1_common_source_ac_matches_ngspice46() {
    let model = bsim4_model(".model n45 nmos level=54 version=4.8 acnqsmod=1 trnqsmod=0 rgatemod=0");
    let deck = format!(
        "* bsim4 ac nqs common-source\n\
         vdd vdd 0 dc 1.2\n\
         vin in 0 dc 0.85 ac 1\n\
         rd vdd out 4k\n\
         m1 out in 0 0 n45 l=45n w=5u\n\
         .ac dec 1 1e6 1e9\n\
         .print ac vm(out) vp(out)\n\
         .end\n\
         {model}"
    );
    let results = run_ac_from_deck(&deck).expect("ACNQSMOD=1 deck runs natively");
    let expected = [
        (1.0e6, -3.702_25, 4.295_778e-5, 11.369_31, 3.141_581),
        (1.0e7, -3.702_25, 4.295_778e-4, 11.369_31, 3.141_477),
        (1.0e8, -3.702_24, 4.295_774e-3, 11.369_31, 3.140_432),
        (1.0e9, -3.701_84, 4.295_381e-2, 11.368_93, 3.129_990),
        (1.0e10, -3.661_32, 4.256_449e-1, 11.331_05, 3.025_858),
        (1.0e11, -1.554_58, 2.232_057, 8.691_610, 2.179_155),
    ];
    assert_ac_real_imag_db_phase_against_ngspice(&results, &expected);
}
```

- [x] **Step 2: Run the focused test to verify red**

Run:

```powershell
cargo test -p rspice-core --test bsim4_native acnqsmod1_common_source_ac_matches_ngspice46 -- --nocapture
```

Expected: fail with `TRNQSMOD/ACNQSMOD=1 is not implemented`.

Xyce 7.10 acceptance note: the local Xyce binary accepts the same deck, but `ACNQSMOD=1` produces the same `.FD.prn` values as `ACNQSMOD=0` on this test circuit. Do not use that output as the AC-NQS numerical target.

### Task 2: Native AC NQS State And Helpers

**Files:**
- Modify: `crates/rspice-core/src/device/mosfet/bsim4v8/eval.rs`
- Modify: `crates/rspice-core/src/device/mosfet/bsim4v8/device.rs`
- Modify: `crates/rspice-core/src/device/mosfet/bsim4v8/mod.rs`

- [x] **Step 1: Store AC NQS scalar outputs**

Extend `Bsim4v8Charge` with:

```rust
pub qchqs: Value,
pub taunet: Value,
```

Populate them in `eval.rs` after intrinsic charges are known:

```rust
ch.qchqs = -(qbulk + qgate);
let cox_wl = model.coxe * p.weff_cv * geom.nf * p.leff_cv;
if (model.trnqs_mod != 0 || model.acnqs_mod != 0) && cox_wl > 0.0 && op.gcrg > 0.0 {
    ch.taunet = cox_wl / op.gcrg;
} else {
    ch.taunet = 0.0;
}
```

- [x] **Step 2: Allow AC NQS construction but keep transient NQS rejected**

Change `Bsim4v8::validate_model` so `ACNQSMOD=1, TRNQSMOD=0` is accepted and `TRNQSMOD=1` remains a typed error.

- [x] **Step 3: Add an AC NQS transform helper**

Add a helper on `Bsim4v8Device` that returns transformed gm/gmbs/gds and intrinsic C rows for one frequency:

```rust
pub fn ac_nqs_terms(&self, charge: &Bsim4v8Charge, omega: Value) -> Option<Bsim4v8AcNqsTerms>
```

It should return `None` unless `self.core.model.acnqs_mod == 1`, `charge.taunet > 0.0`, and `omega > 0.0`.

### Task 3: AC Matrix Integration

**Files:**
- Modify: `crates/rspice-core/src/engine/ac.rs`
- Modify: `crates/rspice-core/src/device/mosfet/bsim4v8/device.rs`

- [x] **Step 1: Do not double-stamp BSIM4 AC NQS as QS**

In `stamp_nonlinear_small_signal_real`, stamp QS BSIM4 devices through the existing path. For AC NQS devices, use a BSIM4 AC-specific complex stamp so the real DC gm/gds block is replaced by the NQS-transformed block.

- [x] **Step 2: Stamp transformed real and imaginary blocks**

The AC-specific stamp should mirror ngspice `b4acld.c`:

```text
T0 = omega * taunet
T2 = 1 / (1 + T0*T0)
T3 = T0 * T2
gmr = gm * T2
gmi = -gm * T3
gdsr = gds * T2
gdsi = -gds * T3
Cddr = cddb * T2
Cddi = cddb * T3 * omega
```

Apply the same transform to the drain/source/gate intrinsic capacitance rows, then add overlap and junction capacitances as the existing QS AC path does.

- [x] **Step 3: Preserve unsupported transient NQS**

Update error messages and docs so `TRNQSMOD=1` remains explicitly unsupported and `ACNQSMOD=1` is no longer described as unported.

### Task 4: Verification

**Files:**
- Modify: `crates/rspice-core/tests/bsim4_native.rs`

- [x] **Step 1: Verify focused red-green**

Run:

```powershell
cargo test -p rspice-core --test bsim4_native acnqsmod1_common_source_ac_matches_ngspice46 -- --nocapture
```

Expected: the AC NQS oracle test passes.

- [x] **Step 2: Verify BSIM4 native suite**

Run:

```powershell
cargo test -p rspice-core --test bsim4_native -- --nocapture
```

Expected: all BSIM4 native tests pass.

- [x] **Step 3: Verify BSIM4 module tests**

Run:

```powershell
cargo test -p rspice-core --lib bsim4v8 -- --nocapture
```

Expected: BSIM4 module tests pass; the existing live-ngspice ignored test stays ignored unless explicitly enabled.

- [x] **Step 4: Verify full ngspice regression only in release**

Run only this command for the full ngspice suite:

```powershell
$env:CARGO_PROFILE_RELEASE_LTO='false'
$env:CARGO_PROFILE_RELEASE_CODEGEN_UNITS='32'
cargo test --release -p rspice-core --test ngspice_regression test_full_ngspice_suite_summary -- --nocapture
```

Expected: output includes `Finished release profile [optimized]` and `TOTAL 113 tests | 113 passed`.
