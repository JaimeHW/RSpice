# CMC Verilog-A To Rust Transpiler Strategy Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

> **Status update (2026-06-25):** The generator/build infrastructure described
> here has started landing behind `rspice-core`'s `veriloga-builtins` feature,
> with generated source materialized under
> `crates/rspice-core/src/device/veriloga_generated/`. This remains
> feature-gated qualification work; the model policy below is still active, but
> do not read this plan as a completed CMC product-support claim.

**Goal:** Stop hand-writing native implementations for devices that have redistributable CMC Verilog-A sources under `models/veriloga/cmc/`, and instead build a Verilog-A to Rust transpiler that generates native Rust device implementations from those sources.

**Architecture:** CMC Verilog-A packages are the source of truth for modern compact models such as PSP, HICUM/L2, MEXTRAM, BSIM-CMG, BSIM-IMG, BSIM-BULK, ASM-HEMT, MVSG, and JUNCAP/diode CMC. The existing Verilog-A parser/semantic pipeline remains the front end; a new Rust codegen backend emits checked-in or cached generated Rust modules with the same simulator-facing contracts as hand-native devices. Historical hand-native CMC slices are not active product code and should only be used as external reference material for generated implementations.

**Tech Stack:** Rust, `rspice-veriloga`, `rspice-core`, generated Rust device modules, CMC Verilog-A sources in `models/veriloga/cmc/`, Xyce 7.10 regression/oracle decks, ngspice 46 where matching behavior is expected, release-only full ngspice regression gate.

---

## Governing Policy

- Do not start new hand-native implementations for any device family that has redistributable CMC Verilog-A source under `models/veriloga/cmc/`.
- Do not restore or extend hand-native CMC slices as the main path unless the work is needed to compare against generated output or support the transpiler itself.
- Generated Rust from CMC Verilog-A is the future native implementation path. "Native" for CMC devices means generated Rust produced by the transpiler, not a manually maintained model port.
- Native hand-written work remains appropriate for non-CMC primitives and infrastructure devices: parser/device-policy wiring, sources, switches, passives, transmission lines, classic Berkeley MOS/JFET/BJT paths, matrix integration, solver work, and validation harnesses.
- Xyce remains the primary compatibility oracle when it supports the target model. ngspice is also used when the model is expected to match ngspice behavior.
- The full ngspice regression suite is run only in `--release`. Focused Rust tests and ordinary integration tests run in debug unless a specific benchmark/release gate requires otherwise.

## Source Scope

All package roots discovered under `models/veriloga/cmc/` are CMC transpiler
inputs. The current shipped set is:

- `models/veriloga/cmc/505p5p0_va` for MEXTRAM 505.5.0.
- `models/veriloga/cmc/ASM-ESD101.1.0_04042025`.
- `models/veriloga/cmc/ASM-HEMT101.6.0_05132026`.
- `models/veriloga/cmc/BSIM_SOI_100.1.1_09152025`.
- `models/veriloga/cmc/BSIM-BULK107.2.1_02112025`.
- `models/veriloga/cmc/BSIM-CMG_112.1.0_04282026`.
- `models/veriloga/cmc/BSIM-IMG_103.0.0_20200102`.
- `models/veriloga/cmc/BSIM-SOI_4.7.0_05192025`.
- `models/veriloga/cmc/diode_cmc_3.0_20250714`.
- `models/veriloga/cmc/hicumL0_v2p1p0_files`.
- `models/veriloga/cmc/hicumL2_v320_files`.
- `models/veriloga/cmc/HiSIM_HV_2.5.1_Release_20230209`.
- `models/veriloga/cmc/HiSIM_SOI_1.5.0_Release_20211008`.
- `models/veriloga/cmc/HiSIM_SOTB_1.3.0_Release_20211116`.
- `models/veriloga/cmc/L_UTSOI_102.9.0_code_package`.
- `models/veriloga/cmc/MOSVAR140`.
- `models/veriloga/cmc/mvsg_cmc_v4.0.0_official`.
- `models/veriloga/cmc/PSP104.1.0_vacode`.
- `models/veriloga/cmc/r2_cmc_v1.0.2`.
- `models/veriloga/cmc/r3_cmc_release1.1.2_2023Jun16`.

If a package is later added under `models/veriloga/cmc/`, it automatically falls under this policy.

---

### Task 1: Mark Stale Hand-Native CMC Plans As Superseded

**Files:**
- Modify: `docs/superpowers/plans/2026-06-20-mextram504-native-dc.md`
- Modify: `docs/superpowers/plans/2026-06-20-hicum2-native-dc.md`
- Modify: `docs/superpowers/plans/2026-06-21-hicum-l2-xyce-self-heated-dc.md`
- Modify: `docs/superpowers/plans/2026-06-20-psp103-swjunasym.md`

- [x] **Step 1: Add a supersession notice to each plan**

Insert this block immediately after each `# ... Implementation Plan` heading:

```markdown
> **Superseded on 2026-06-24:** Do not continue this as a hand-native CMC model implementation plan. CMC models with Verilog-A sources under `models/veriloga/cmc/` are now implemented through the Verilog-A to Rust transpiler strategy in `docs/superpowers/plans/2026-06-24-cmc-veriloga-transpiler-strategy.md`. Existing native code/tests from this slice may remain as validation references or compatibility scaffolding, but new model coverage should target generated Rust from Verilog-A.
```

- [x] **Step 2: Verify the notices are present**

Run:

```powershell
rg -n "Superseded on 2026-06-24|Verilog-A to Rust transpiler strategy" docs/superpowers/plans
```

Expected: the new strategy file and each superseded CMC hand-native plan appears in the output.

### Task 2: Update The Engineering Roadmap

**Files:**
- Modify: `docs/ROADMAP.md`

- [x] **Step 1: Replace WS2 wording with transpiler-first language**

In `WS2 -- Compact-model program`, keep the existing validation ambition but state that CMC models are not hand-ported. The section must say:

```markdown
## WS2 -- Compact-model program (Verilog-A to generated Rust)

**Goal:** close the modern-silicon model gap (FinFET/FDSOI/PSP/HICUM/MEXTRAM/GaN) without hand-porting CMC compact models. CMC Verilog-A packages under `models/veriloga/cmc/` are the source of truth; the strategic deliverable is a Verilog-A to Rust transpiler that emits native Rust device implementations from those sources.
```

- [x] **Step 2: Add a hard policy bullet to M2.1**

Add:

```markdown
- Policy: no new hand-native CMC model ports. Historical hand-native CMC slices are reference material only; generated Rust from the Verilog-A source is the active implementation path.
```

- [x] **Step 3: Adjust M2.2 acceptance wording**

Make the model list explicitly say each CMC model lands as generated Rust from Verilog-A, with Xyce/ngspice oracle validation where applicable.

- [x] **Step 4: Keep M2.3 for non-CMC/native gaps**

Rename M2.3 to:

```markdown
### M2.3 -- Hand-native completion only where the VA/transpiler route does not cover
```

Expected: hand-native BSIM4 legacy work remains allowed, while PSP/HICUM/MEXTRAM/BSIM-CMG style CMC work is moved to the generated path.

### Task 3: Document Model Library Meaning

**Files:**
- Modify: `models/README.md`
- Modify: `README.md`
- Modify: `crates/rspice-core/README.md`

- [x] **Step 1: Update `models/README.md`**

Add after the `veriloga/cmc/` paragraph:

```markdown
These CMC sources are not a staging area for hand-written native ports. They are the canonical inputs for the planned Verilog-A to Rust transpiler; generated Rust devices should preserve the upstream source package identity and license/notice attribution.
```

- [x] **Step 2: Update top-level support language**

Where the README support table mentions CMC-derived model families such as PSP, MEXTRAM, HICUM/L2, BSIM-CMG, BSIM-BULK, ASM-HEMT, or MVSG, clarify that future full CMC coverage comes from generated Rust from Verilog-A. Keep already-supported legacy/native paths described factually, but do not present hand-native CMC ports as the ongoing strategy.

- [x] **Step 3: Update `crates/rspice-core/README.md` extension-point text**

Add a note that the stable simulator-facing API for generated CMC Rust devices should match the existing native-device storage/stamping contracts, but source ownership remains with `rspice-veriloga` code generation.

### Task 4: Define The Transpiler Work Breakdown

**Files:**
- Create: `docs/superpowers/plans/2026-06-24-veriloga-rust-codegen-core.md`

- [x] **Step 1: Write the codegen-core plan**

Create a follow-up plan whose first implementation milestone covers:

- Verilog-A AST lowering into a typed model IR.
- Parameter/default/range extraction.
- Internal node and branch declaration extraction.
- Contribution classification for DC, AC, transient charge, and noise.
- Rust code generation for a minimal generated two-terminal diode-like model fixture.
- Golden tests that compare interpreted Verilog-A VM output and generated Rust output on the same AST.

- [x] **Step 2: Keep model-specific CMC work blocked on the codegen core**

Do not create PSP/HICUM/MEXTRAM/BSIM-CMG generated model tasks until the codegen-core plan has a passing generated fixture and a stable simulator-facing trait contract.

### Task 5: Verification

**Files:**
- Documentation-only unless this plan is executed into codegen work.

- [x] **Step 1: Run documentation/search verification**

Run:

```powershell
rg -n "hand-native CMC|hand-written native|Verilog-A to Rust|transpiler|models/veriloga/cmc" README.md crates/rspice-core/README.md models/README.md docs/ROADMAP.md docs/superpowers/plans
```

Expected: current docs consistently say CMC Verilog-A packages are transpiler inputs, and stale hand-native CMC plans are marked superseded.

- [x] **Step 2: Do not run simulator tests for doc-only changes**

No `cargo test` is required for this strategy-doc update. When codegen work starts, run focused debug tests for normal Rust verification and reserve `--release` for the full ngspice regression suite.

---

## Self-Review

- Spec coverage: this plan implements the requested pivot away from hand-native CMC ports and toward generated native Rust from CMC Verilog-A sources.
- Placeholder scan: no `TBD`, `TODO`, or vague implementation placeholders remain.
- Type consistency: the plan consistently names the strategy as "Verilog-A to Rust transpiler" and keeps CMC model coverage tied to `models/veriloga/cmc/`.
