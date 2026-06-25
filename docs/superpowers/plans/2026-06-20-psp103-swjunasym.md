# PSP103 SWJUNASYM Implementation Plan

> **Superseded on 2026-06-24:** Do not continue this as a hand-native CMC model implementation plan. CMC models with Verilog-A sources under `models/veriloga/cmc/` are now implemented through the Verilog-A to Rust transpiler strategy in `docs/superpowers/plans/2026-06-24-cmc-veriloga-transpiler-strategy.md`. Any hand-native code/tests from this slice should be removed from active code paths; keep only historical notes or external validation data, and target new model coverage at generated Rust from Verilog-A.

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add native PSP103 `SWJUNCAP=3 SWJUNASYM=1` DC junction-current support validated against Xyce 7.10.

**Architecture:** Reuse the existing native `Juncap200` non-express core. Keep PSP103 source/drain per-finger geometry in `Psp103Model::derive_junctions`, keep PSP `MULT*NF` terminal scaling outside the JUNCAP core, and map drain-suffixed model parameters into the drain core only when `SWJUNASYM=1`.

**Tech Stack:** Rust, `rspice-core`, native PSP103/JUNCAP200 device code, Xyce 7.10 oracle rows from `C:\Users\James\Desktop\xyce_oracle_work\psp103_swjunasym_oracle`.

---

### Task 1: Add Xyce-Backed Asymmetric PSP103 DC Tests

**Files:**
- Modify: `crates/rspice-core/tests/psp103_native.rs`

- [x] **Step 1: Write the failing NMOS test**

Add helper `psp103_nmos_idvg_swjuncap3_swjunasym1_deck(vd: f64)` by replacing:

```rust
"SWIGATE=0 SWIMPACT=0 SWGIDL=0 SWJUNCAP=0"
```

with:

```rust
"SWIGATE=1 SWIMPACT=1 SWGIDL=1 SWJUNCAP=3 SWJUNASYM=1"
```

Use the same probe topology as the existing PSP103 IDVG tests and `AS=5e-12 AD=5e-12 PS=21e-6 PD=21e-6`.

Add test rows for `Vd=1.0`, `TEMP=27` from `psp_asym_nmos_idvg0.cir.prn`:

```rust
(0usize, 3.840_851_57e-8, -5.389_790_98e-10, 4.976_648_99e-10, -3.836_720_15e-8),
(100, 2.185_204_49e-7, -2.034_797_98e-13, -1.801_448_71e-7, -3.837_537_45e-8),
(150, 1.528_411_06e-3, 9.806_588_95e-10, -1.528_364_98e-3, -4.706_351_85e-8),
(220, 8.124_423_07e-3, 1.054_573_26e-8, -8.124_395_24e-3, -3.837_044_45e-8),
```

- [x] **Step 2: Run test to verify it fails**

Run:

```powershell
cargo test -p rspice-core --test psp103_native psp103_level103_asym_nmos_idvg_swjuncap3_swjunasym1_matches_xyce710_dc_subset -- --nocapture
```

Expected: fail with the current `SWJUNASYM=0 symmetric junction parameters so far` unsupported-slice error.

- [x] **Step 3: Add PMOS coverage**

Add helper `psp103_pmos_idvg_swjuncap3_swjunasym1_deck(vd: f64)` with the same switch replacement and PMOS sweep direction.

Add test rows for `Vd=-1.0`, `TEMP=27` from `psp_asym_pmos_idvg0.cir.prn`:

```rust
(0usize, -3.840_143_97e-8, 5.248_269_72e-10, -4.905_888_38e-10, 3.836_720_15e-8),
(99, -9.758_403_53e-8, 7.168_166_41e-13, 5.921_342_92e-8, 3.836_988_92e-8),
(149, -1.179_563_58e-3, -9.110_325_81e-10, 1.179_516_20e-3, 4.828_827_99e-8),
(220, -9.758_940_06e-3, -1.029_771_58e-8, 9.758_911_99e-3, 3.836_952_88e-8),
```

- [x] **Step 4: Run test to verify it fails**

Run:

```powershell
cargo test -p rspice-core --test psp103_native psp103_level103_asym_pmos_idvg_swjuncap3_swjunasym1_matches_xyce710_dc_subset -- --nocapture
```

Expected: fail with the same unsupported `SWJUNASYM=1` guard.

### Task 2: Implement Drain-Suffixed JUNCAP Mapping

**Files:**
- Modify: `crates/rspice-core/src/device/mosfet/psp103.rs`

- [x] **Step 1: Remove the SWJUNASYM rejection**

In `Psp103Model::validate_supported_dc_slice`, keep the `SWJUNCAP=3` and `SWJUNEXP=0` guards, but allow `SWJUNASYM` values clipped to `0` or `1`.

- [x] **Step 2: Add a drain-parameter remap helper**

Add a helper that clones model params for the drain JUNCAP core and maps these drain names back to the source/generic names expected by `Juncap200`:

```rust
[
    ("CJORBOTD", "CJORBOT"), ("CJORSTID", "CJORSTI"), ("CJORGATD", "CJORGAT"),
    ("VBIRBOTD", "VBIRBOT"), ("VBIRSTID", "VBIRSTI"), ("VBIRGATD", "VBIRGAT"),
    ("PBOTD", "PBOT"), ("PSTID", "PSTI"), ("PGATD", "PGAT"),
    ("PHIGBOTD", "PHIGBOT"), ("PHIGSTID", "PHIGSTI"), ("PHIGGATD", "PHIGGAT"),
    ("IDSATRBOTD", "IDSATRBOT"), ("IDSATRSTID", "IDSATRSTI"), ("IDSATRGATD", "IDSATRGAT"),
    ("CSRHBOTD", "CSRHBOT"), ("CSRHSTID", "CSRHSTI"), ("CSRHGATD", "CSRHGAT"),
    ("XJUNSTID", "XJUNSTI"), ("XJUNGATD", "XJUNGAT"),
    ("CTATBOTD", "CTATBOT"), ("CTATSTID", "CTATSTI"), ("CTATGATD", "CTATGAT"),
    ("MEFFTATBOTD", "MEFFTATBOT"), ("MEFFTATSTID", "MEFFTATSTI"), ("MEFFTATGATD", "MEFFTATGAT"),
    ("CBBTBOTD", "CBBTBOT"), ("CBBTSTID", "CBBTSTI"), ("CBBTGATD", "CBBTGAT"),
    ("FBBTRBOTD", "FBBTRBOT"), ("FBBTRSTID", "FBBTRSTI"), ("FBBTRGATD", "FBBTRGAT"),
    ("STFBBTBOTD", "STFBBTBOT"), ("STFBBTSTID", "STFBBTSTI"), ("STFBBTGATD", "STFBBTGAT"),
    ("VBRBOTD", "VBRBOT"), ("VBRSTID", "VBRSTI"), ("VBRGATD", "VBRGAT"),
    ("PBRBOTD", "PBRBOT"), ("PBRSTID", "PBRSTI"), ("PBRGATD", "PBRGAT"),
]
```

- [x] **Step 3: Wire the helper in `derive_junctions`**

Use the original parameter clone for the source JUNCAP core. If `SWJUNASYM=1`, build a second remapped clone for the drain JUNCAP core; otherwise reuse the source/generic clone for the drain.

- [x] **Step 4: Run focused tests**

Run:

```powershell
cargo test -p rspice-core --test psp103_native psp103_level103_asym_nmos_idvg_swjuncap3_swjunasym1_matches_xyce710_dc_subset -- --nocapture
cargo test -p rspice-core --test psp103_native psp103_level103_asym_pmos_idvg_swjuncap3_swjunasym1_matches_xyce710_dc_subset -- --nocapture
cargo test -p rspice-core --test psp103_native -- --nocapture
```

Expected: all pass.

### Task 3: Verification

**Files:**
- Modify: no new source files expected

- [x] **Step 1: Format check**

Run:

```powershell
cargo fmt --all -- --check
```

Expected: pass.

- [x] **Step 2: Release-focused PSP103 test**

Run:

```powershell
cargo test --release -p rspice-core --test psp103_native -- --nocapture
```

Expected: pass.

- [x] **Step 3: Full ngspice regression release gate**

Run only in release mode:

```powershell
$env:CARGO_PROFILE_RELEASE_LTO='false'
$env:CARGO_PROFILE_RELEASE_CODEGEN_UNITS='32'
$env:NGSPICE_EXE='C:\Users\James\Desktop\ngspice-46-release\Spice64\bin\ngspice_con.exe'
cargo test --release -p rspice-core --test ngspice_regression test_full_ngspice_suite_summary -- --nocapture
```

Expected: pass. Do not run this full suite without `--release`.

### Verification Evidence

- `cargo test -p rspice-core --test psp103_native -- --nocapture`: `53 passed; 0 failed`.
- `cargo fmt --all -- --check`: passed.
- `cargo test --release -p rspice-core --test psp103_native -- --nocapture`: `53 passed; 0 failed`.
- `cargo test --release -p rspice-core --test ngspice_regression test_full_ngspice_suite_summary -- --nocapture` with `NGSPICE_EXE=C:\Users\James\Desktop\ngspice-46-release\Spice64\bin\ngspice_con.exe`: `113 passed; 0 failed; 0 skipped`.
