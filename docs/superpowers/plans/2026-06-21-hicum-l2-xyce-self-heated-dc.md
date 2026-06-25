# HICUM/L2 Xyce Five-Terminal DC Implementation Plan

> **Superseded on 2026-06-24:** Do not continue this as a hand-native CMC model implementation plan. CMC models with Verilog-A sources under `models/veriloga/cmc/` are now implemented through the Verilog-A to Rust transpiler strategy in `docs/superpowers/plans/2026-06-24-cmc-veriloga-transpiler-strategy.md`. Any hand-native code/tests from this slice should be removed from active code paths; keep only historical notes or external validation data, and target new model coverage at generated Rust from Verilog-A.

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add the first Xyce-primary native HICUM/L2 `BJT LEVEL=234 VERSION=2.34/2.40` five-terminal DC path without falling back to VBIC, legacy BJT, Verilog-A, or the simplified ngspice-only three-terminal HICUM2 topology.

**Architecture:** Keep `LEVEL=8 VERSION=2.40` ngspice HICUM2 as the existing simple external collector/base/emitter slice. Add a separate Xyce HICUM/L2 topology path that allocates the ADMS internal nodes (`ci`, `ei`, `bp`, `bi`, `si`, thermal, and NQS placeholders where needed), stamps static branch currents and series resistors in the same branch orientation as Xyce, and solves the self-heating DC thermal branch when `FLSH=1`. Dynamic charge, AC, noise, NQS, and HICUM/L0 remain separate slices.

**Tech Stack:** Rust, `rspice-core`, native nonlinear device storage/stamping, Xyce 7.10 NORAD oracle at `C:/Users/James/Downloads/Xyce-7.10-NORAD/bin/Xyce.exe`, Xyce ADMS source `C:/Users/James/Downloads/Xyce-7.10.0/Xyce-7.10/utils/ADMS/examples/hicum/hicumL2V2p4p0.va`, Xyce regression deck `C:/Users/James/Downloads/Xyce_Regression-master/Xyce_Regression-master/Netlists/HICUM/fgum_dc_npn_full_sh.cir`, release-only ngspice regression gate.

---

### Implementation Status

- Completed the first native Xyce `LEVEL=234 VERSION=2.34/2.40` five-terminal DC slice with static internal nodes `ci`, `ei`, `bp`, `bi`, `si` and the external thermal node.
- Kept ngspice `LEVEL=8 VERSION=2.40` on the existing native external C/B/E HICUM2 path.
- Added Xyce regression-oracle checks for `V(base)=0.8` at `V(coll)=0.5`, `1.0`, and `1.5`.
- Still out of scope for this slice: AC, transient charge, noise, NQS/excess-phase nodes, HICUM/L0, and PNP.

---

### Current Evidence

- Xyce rejects three-terminal and four-terminal top-level `LEVEL=234` HICUM instances; the registered ADMS model is the five-terminal form `q1 coll base emit subs therm mymodel`.
- The official Xyce `fgum_dc_npn_full_sh.cir` model has `FLSH=1`, `RTH=1113.4`, `CTH=6.841e-12`, nonzero `RBI0`, `RBX`, `RE`, `RCX`, perimeter junction currents, and substrate-capable pins. It is not equivalent to the existing RSpice `LEVEL=8 VERSION=2.40` external C/B/E-only route.
- At `TEMP=27`, `V(base)=0.8`, `V(coll)=0.5`, the Xyce regression output reports `I(V_COLL)=1.17006327e-03` and `I(V_BASE)=3.17566867e-06`.
- Running the same model card through the existing RSpice `LEVEL=8 VERSION=2.40` path gives about `I(C)=1.223e-03` and `I(B)=1.715e-06`, so a simple selector alias would be physically wrong.
- `crates/rspice-core/src/device/semiconductor/hicum2.rs` currently stamps only the external collector/base/emitter 3x3 Jacobian.
- `crates/rspice-core/src/device/semiconductor/hicum2/eval.rs` already contains a useful partial HICUM/L2 current evaluator, but it evaluates only `V(bi,ei)` and `V(ci,ei)` and omits the full ADMS branch network.

### Non-Goals

- Do not enable Xyce `LEVEL=234` by routing it to the existing `Hicum2` external C/B/E stamp.
- Do not approximate the Xyce regression by fitting terminal currents.
- Do not add a Verilog-A runtime fallback.
- Do not implement AC, transient charge, noise, HICUM/L0, or NQS in this slice.
- Do not run the full ngspice regression suite without `--release`.

---

### Task 1: Add The RED Xyce HICUM Five-Terminal Oracle Test

**Files:**
- Modify: `crates/rspice-core/tests/hicum2_native.rs`
- Read: `C:/Users/James/Downloads/Xyce_Regression-master/Xyce_Regression-master/Netlists/HICUM/fgum_dc_npn_full_sh.cir`
- Read: `C:/Users/James/Downloads/Xyce_Regression-master/Xyce_Regression-master/OutputData/HICUM/fgum_dc_npn_full_sh.cir.prn`

- [ ] **Step 1: Add the Xyce model card constant**

Add `HICUM_L2_XYCE_FULL_SH_MODEL` to `hicum2_native.rs` by copying only the `.model mymodel npn level=234 version=2.34` block from the Xyce regression deck through the line `+ zetahjei= -0.5`. Keep the model card in the test file at first so the RED test is self-contained.

- [ ] **Step 2: Add a direct five-terminal deck helper**

Add this helper:

```rust
fn hicum_l2_xyce234_full_sh_direct_op_deck(vb: f64, vc: f64) -> String {
    format!(
        "* Xyce 7.10 HICUM/L2 full-sh five-terminal DC oracle\n\
         .options temp=27\n\
         vc coll 0 dc {vc:.15}\n\
         vb base 0 dc {vb:.15}\n\
         ve emit 0 dc 0\n\
         vs subs 0 dc 0\n\
         ith therm 0 dc 0\n\
         q1 coll base emit subs therm mymodel\n\
         {HICUM_L2_XYCE_FULL_SH_MODEL}\n\
         .op\n\
         .end\n"
    )
}
```

- [ ] **Step 3: Add the failing oracle test**

Add:

```rust
#[test]
fn hicum_l2_level234_five_terminal_full_sh_matches_xyce710_dc_point() {
    let result = run_op(&hicum_l2_xyce234_full_sh_direct_op_deck(0.8, 0.5));

    assert_rel_close("Xyce HICUM I(coll)", -source_current(&result, "vc"), 1.17006327e-03, 5.0e-4);
    assert_rel_close("Xyce HICUM I(base)", -source_current(&result, "vb"), 3.17566867e-06, 5.0e-4);
}
```

- [ ] **Step 4: Run the RED test**

Run:

```powershell
cargo test -p rspice-core --test hicum2_native hicum_l2_level234_five_terminal_full_sh_matches_xyce710_dc_point -- --nocapture
```

Expected: FAIL with the current `LEVEL=234` HICUM fail-closed error. If it fails earlier due parser terminal handling, fix the parser first and keep the builder fail-closed.

---

### Task 2: Split HICUM Device Topology For Xyce Without Breaking Ngspice

**Files:**
- Modify: `crates/rspice-core/src/device/semiconductor/hicum2.rs`
- Modify: `crates/rspice-core/src/circuit/storage/nonlinear.rs`
- Modify: `crates/rspice-core/src/engine/matrix.rs`
- Modify: `crates/rspice-core/src/engine/builder.rs`
- Test: `crates/rspice-core/tests/bjt_level_policy.rs`

- [ ] **Step 1: Add topology enum**

Add:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hicum2Topology {
    NgspiceExternalCbe,
    XyceFiveTerminal,
}
```

Extend `Hicum2` with `topology: Hicum2Topology`, optional `node_thermal`, and optional internal node ids `ci`, `ei`, `bp`, `bi`, `si`, `xf1`, `xf2`, `xf`, `n1`, and `n2`.

- [ ] **Step 2: Preserve existing constructor**

Keep `Hicum2::new(...)` as a wrapper that builds `NgspiceExternalCbe`, so the existing ngspice HICUM test and policy route do not change.

- [ ] **Step 3: Add Xyce constructor**

Add:

```rust
pub fn new_xyce_five_terminal(
    name: String,
    collector: NodeId,
    base: NodeId,
    emitter: NodeId,
    substrate: NodeId,
    thermal: NodeId,
    internals: Hicum2InternalNodes,
    model: Hicum2Model,
) -> Self
```

where `Hicum2InternalNodes` holds the internal node ids listed above.

- [ ] **Step 4: Reserve Xyce matrix positions**

In `engine/matrix.rs`, reserve a dense static block over the external nodes and the internal electrical/thermal nodes for `XyceFiveTerminal`. Keep the current 3x3 reservation for `NgspiceExternalCbe`.

- [ ] **Step 5: Keep Xyce route fail-closed until stamping exists**

Update `bjt_level_policy.rs` so `LEVEL=234 VERSION=2.34` still fails closed, but the message says the remaining gap is `Xyce five-terminal HICUM/L2 topology`, not "no native HICUM implementation" generically.

---

### Task 3: Port Static Xyce Branch Extraction

**Files:**
- Modify: `crates/rspice-core/src/device/semiconductor/hicum2/eval.rs`
- Modify: `crates/rspice-core/src/device/semiconductor/hicum2.rs`
- Test: `crates/rspice-core/src/device/semiconductor/hicum2/eval.rs`

- [ ] **Step 1: Add branch-current result type**

Add:

```rust
#[derive(Debug, Clone, Copy, Default)]
pub struct Hicum2BranchOp {
    pub ibiei: Value,
    pub ibici: Value,
    pub iciei: Value,
    pub ieici: Value,
    pub ibpei: Value,
    pub ibpci: Value,
    pub ibbp: Value,
    pub ibpbi: Value,
    pub icic: Value,
    pub ieie: Value,
    pub isis: Value,
    pub isici: Value,
    pub ibpsi: Value,
    pub itherm: Value,
}
```

- [ ] **Step 2: Evaluate from branch voltages**

Add `evaluate_xyce_static_dc(model, branch_voltages)` that derives the same core quantities as `evaluate_currents`, but uses Xyce branch voltages:

```text
Vbiei = type * V(bi, ei)
Vbici = type * V(bi, ci)
Vciei = type * V(ci, ei)
Vbpei = type * V(bp, ei)
Vbpci = type * V(bp, ci)
Vsici = type * V(si, ci)
Vsc   = type * V(s, c)
Vthermal = V(tnode)
```

For the first GREEN target, include these static ADMS contributions:

```text
I(br_biei) += type*(ibei + irei + ibh_rec) + gmin*V(bi,ei)
I(br_bici) += type*(ibci - iavl) + gmin*V(bi,ci)
I(br_ciei) += type*Itxf + gmin*V(ci,ei)
I(br_eici) += type*itr
I(br_bpei) += type*(ibep + irep - ibet when tunode==1)
I(br_bpci) += type*ijbcx
I(br_bpsi) += type*HSI_Tsu
I(br_sici) += type*ijsc
I(br_bbp_i) += V(b,bp)/rbx_t
I(br_bpbi_i) += V(bp,bi)/rbi
I(br_cic_i) += V(ci,c)/rcx_t
I(br_eie_i) += V(ei,e)/re_t
I(br_sis_i) += V(si,s)/rsu
I(br_sht) += V(tnode)/rth_t - pterm
```

Do not include `ddt(...)`, AC, or noise terms in this DC slice.

- [ ] **Step 3: Add evaluator unit tests for branch extraction**

Use a zero-resistance model to prove `evaluate_xyce_static_dc` collapses to the existing external evaluator when `rbx`, `rbi0`, `rcx`, `re`, `rsu`, and `flsh` are zero.

Run:

```powershell
cargo test -p rspice-core --lib hicum2 -- --nocapture
```

Expected: existing ngspice HICUM tests still pass and the new branch-extraction test passes.

---

### Task 4: Stamp Xyce Five-Terminal DC Branches

**Files:**
- Modify: `crates/rspice-core/src/device/semiconductor/hicum2.rs`
- Test: `crates/rspice-core/tests/hicum2_native.rs`

- [ ] **Step 1: Add finite-difference branch Jacobian**

For `XyceFiveTerminal`, compute finite-difference derivatives of the branch-current vector with respect to the Xyce node vector:

```text
[c, b, e, s, tnode, ci, ei, bp, bi, si]
```

Use the same perturbation rule as the existing evaluator. Stamp `I(pos, neg)` branches with equivalent conductance and RHS contributions.

- [ ] **Step 2: Stamp zero-resistance branch voltage collapses**

When Xyce source uses `V(branch) <+ 0.0` for zero resistors, stamp a large conductance tie between the branch nodes instead of silently leaving them disconnected. Use the same conservative conductance already used for stiff internal native ties elsewhere in `rspice-core`.

- [ ] **Step 3: Stamp self-heating branch**

For `FLSH=1` and `RTH >= MIN_R`, stamp:

```text
I(thermal, 0) += V(thermal)/rth_t - pterm
```

For `FLSH=0` or `RTH < MIN_R`, stamp `V(thermal)/MIN_R` so the external thermal node remains numerically anchored like Xyce.

- [ ] **Step 4: Run the Xyce RED/GREEN point**

Run:

```powershell
cargo test -p rspice-core --test hicum2_native hicum_l2_level234_five_terminal_full_sh_matches_xyce710_dc_point -- --nocapture
```

Expected after implementation: PASS within `5e-4` relative tolerance for collector and base source currents.

---

### Task 5: Enable Builder Route And Preserve Fail-Closed Boundaries

**Files:**
- Modify: `crates/rspice-core/src/engine/builder.rs`
- Modify: `crates/rspice-core/tests/bjt_level_policy.rs`

- [ ] **Step 1: Route exact Xyce HICUM selectors**

Enable native routing for:

```text
LEVEL=234 VERSION=2.34 NPN five-terminal
LEVEL=234 VERSION=2.40 NPN five-terminal
```

Allocate the internal nodes using stable names:

```text
{q}.__ci.internal
{q}.__ei.internal
{q}.__bp.internal
{q}.__bi.internal
{q}.__si.internal
{q}.__xf1.internal
{q}.__xf2.internal
{q}.__xf.internal
{q}.__n1.internal
{q}.__n2.internal
```

- [ ] **Step 2: Keep unsupported forms explicit**

Keep these fail-closed:

```text
LEVEL=234 PNP
LEVEL=234 with fewer than five terminals
LEVEL=234 with instance params
LEVEL=234 with VERSION other than 2.34 or 2.40
LEVEL=8 five-terminal thermal/self-heated ngspice syntax until ngspice thermal QA is added
```

- [ ] **Step 3: Update policy tests**

Change `xyce_hicum_level234_five_terminal_fails_closed_on_hicum_level` into a native-route assertion using `device_summary().count_by_kind("HICUM2")`.

Add separate fail-closed tests for PNP, wrong version, and fewer-than-five-terminal Xyce HICUM forms.

Run:

```powershell
cargo test -p rspice-core --test bjt_level_policy hicum -- --nocapture
```

Expected: all HICUM policy tests pass.

---

### Task 6: Expand Xyce Regression Coverage

**Files:**
- Modify: `crates/rspice-core/tests/hicum2_native.rs`
- Optional create: `crates/rspice-core/tests/testdata/hicum_l2_xyce234_fgummel_full_sh.dat`

- [ ] **Step 1: Add selected sweep rows**

Add representative rows from Xyce `fgum_dc_npn_full_sh.cir.prn` for `V(coll)=0.5`, `1.0`, and `1.5`, including:

```text
V(coll)=0.5  V(base)=0.8  I(V_COLL)=1.17006327e-03  I(V_BASE)=3.17566867e-06
V(coll)=1.0  V(base)=0.8  I(V_COLL)=1.19470210e-03  I(V_BASE)=3.29432084e-06
V(coll)=1.5  V(base)=0.8  I(V_COLL)=1.22019265e-03  I(V_BASE)=2.95222093e-06
```

- [ ] **Step 2: Add full selected sweep test**

Use `run_dc_sweep2_with_abort` if available for the `.step V_COLL list .5 1 1.5` plus `.dc V_BASE 0.3 1.05 0.02` shape. Otherwise, run three independent sweeps with fixed collector voltage.

- [ ] **Step 3: Run focused HICUM verification**

Run:

```powershell
cargo test -p rspice-core --lib hicum2 -- --nocapture
cargo test -p rspice-core --test hicum2_native -- --nocapture
cargo test -p rspice-core --test bjt_level_policy hicum -- --nocapture
```

Expected: all pass.

---

### Task 7: Final Verification

**Files:**
- Modify: no new source files expected

- [ ] **Step 1: Formatting**

Run:

```powershell
cargo fmt --all -- --check
git diff --check
```

Expected: pass. If `git diff --check` reports pre-existing CRLF warnings outside HICUM files, record them separately and verify no new HICUM whitespace errors were introduced.

- [ ] **Step 2: Xyce-backed model tests**

Run:

```powershell
cargo test -p rspice-core --test hicum2_native -- --nocapture
cargo test -p rspice-core --test mextram504_native -- --nocapture
cargo test -p rspice-core --test bsimcmg111_native -- --nocapture
cargo test -p rspice-core --test psp103_native -- --nocapture
```

Expected: pass.

- [ ] **Step 3: Full ngspice regression release gate**

Run only in release mode:

```powershell
$env:CARGO_PROFILE_RELEASE_LTO='false'
$env:CARGO_PROFILE_RELEASE_CODEGEN_UNITS='32'
$env:NGSPICE_EXE='C:\Users\James\Desktop\ngspice-46-release\Spice64\bin\ngspice_con.exe'
cargo test --release -p rspice-core --test ngspice_regression test_full_ngspice_suite_summary -- --nocapture
```

Expected: pass. Non-release full-suite runs are invalid evidence for this gate.

---

### Self-Review

- Spec coverage: this plan addresses the actual Xyce `LEVEL=234` gap rather than aliasing it to the ngspice-only HICUM2 slice. It preserves fail-closed behavior for unsupported HICUM forms and keeps ngspice release-suite discipline explicit.
- Placeholder scan: no `TBD`, `TODO`, or vague "handle edge cases" placeholders remain.
- Type consistency: `Hicum2Topology`, `Hicum2InternalNodes`, `Hicum2BranchOp`, `evaluate_xyce_static_dc`, and `new_xyce_five_terminal` are named consistently across the tasks.
