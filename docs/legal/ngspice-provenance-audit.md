# ngspice Provenance Audit

**Date:** 2026-06-10
**Scope:** RSpice repository (`C:\Users\James\Desktop\RSpice`) audited against the local ngspice-46 source tree (`C:\Users\James\Desktop\ngspice-46-release\ngspice-46`).
**Status:** Engineering research, not legal advice. Prepared by automated/manual code survey; have counsel review before relying on it for licensing or distribution decisions.

**2026-06-18 KLU-class solver addendum:** the current tree contains RSpice's
own KLU-class real solver backend in `crates/rspice-core/src/solver/klu.rs`.
A source survey of that module and its call sites found that no SuiteSparse
KLU source or binding is vendored. The implementation is in-tree Rust using
faer's permissively licensed AMD ordering support, with KLU-class terminology
limited to algorithm-family descriptions (sparse circuit LU, diagonal-biased
pivoting, reusable symbolic/pivot structure, and future BTF-style blocking).
The prohibition on reading or translating ngspice `src/maths/KLU` remains in
force.

---

## 1. Methodology

1. Case-insensitive content search of the entire RSpice tree for: `ngspice`, `vsrcload`, `spice3`, `ported from`, `adapted from`, `based on`, `translated from`, `transcribed`, `faithful port`, `direct port`, `numparam`, `xspice`, `KLU`, `berkeley` (996 occurrences across 200+ files; `target/` excluded via gitignore).
2. Every hit in `crates/`, `models/`, `benchmarks/`, `docs/`, and `tests/` was classified:
   - **(a) behavioral parity** — test oracle, output-format matching, semantics matching, constants that are facts (defaults, tolerances). No license consequence.
   - **(b) algorithm studied / translated at equation level** from a named ngspice file. The named file's license header was read in the local ngspice-46 tree.
   - **(c) direct port / verbatim copy** of code or files. License of the source file attaches.
3. ngspice's `COPYING` (root of ngspice-46) was read in full to map its mixed-license structure.
4. Vendored assets checked: `tests/ngspice/` and `tests/xyce/` deck trees, `models/`, `assets/`, embedded fonts.
5. Direct dependencies of all seven workspace crates were enumerated from `Cargo.toml` files and license fields confirmed from the local cargo registry cache.

## 2. ngspice-46 license structure (from its `COPYING`)

ngspice is **Modified BSD (BSD-3-Clause)** for *"all of its source code, test and example files"* **except**:

| Path | License |
|---|---|
| `src/maths/KLU` | **LGPLv2** |
| `src/frontend/numparam` | **LGPLv2 or newer** |
| `src/xspice/icm/table` | **GPLv2 or newer** |
| `src/tclspice.c` | LGPLv2 |
| `src/osdi` | MPL-2.0 |
| `src/xspice` (rest) | public domain (Georgia Tech) |
| `src/spicelib/devices/ndev` | public domain |
| `src/maths/sparse` | MIT-style |
| `m4/` | DFSG-compatible |
| ngspice manual | CC-BY-SA 4.0 |

**Key audit result after the 2026-06-18 KLU-class solver addendum: identified code-level derivation traces to permissive upstream sources: ngspice Modified-BSD-covered files, plus the native BSIM4 v4.8 port whose upstream BSIM4 4.8.3 files carry explicit UC Berkeley BSIM4 / Educational Community License 2.0 notices and B4TERMS_OF_USE. No SuiteSparse KLU source or binding is vendored, and nothing was taken from ngspice KLU, numparam, xspice/icm/table, tclspice, osdi, or the ngspice manual.**

## 3. Findings table

Risk legend: **none** = behavioral only; **low** = BSD-3 attribution required (now provided via `NOTICE`); **flag** = needs follow-up.

### Class (c) — direct ports / verbatim copies (BSD-3 attribution required)

| RSpice file(s) | ngspice source | License (header checked) | Risk | Action |
|---|---|---|---|---|
| `crates/rspice-core/src/device/mosfet/b3soi/**` (~13.9k lines: `dd/`, `fd/`, `pd/` — `eval.rs` "faithful port", `temp.rs` "faithful port", `params.rs` "transcribed", `common.rs` constants) | `src/spicelib/devices/bsim3soi_dd/`, `bsim3soi_fd/`, `bsim3soi_pd/` (`b3soi{dd,fd,pd}ld.c`, `*temp.c`, `*set.c`, `*.c` param tables) | Modified BSD — "Copyright 1990/1999 Regents of the University of California", authors Fung/Sinitsky/Tang/Liu/Su et al. | low | Attribution added to `NOTICE`. Recommended: add the UC Regents copyright line to the module headers of the ported files. |
| `crates/rspice-core/src/device/mosfet/bsim4v8/**` (native BSIM4 v4.8 path for MOS `LEVEL=14/54`) | `src/spicelib/devices/bsim4/` (`b4.c`, `b4set.c`, `b4temp.c`, `b4check.c`, `b4ld.c`, `b4acld.c`, `b4geo.c`, `bsim4def.h`, etc.) | Educational Community License 2.0 plus `B4TERMS_OF_USE`; "Copyright (c) 2025 University of California"; project directors Prof. Sayeef Salahuddin and Prof. Chenming Hu | low | Attribution added to `NOTICE`; keep UC Berkeley BSIM Research Group acknowledgement in product documentation and preserve upstream copyright notices on distributed copies/modifications. |
| `crates/rspice-core/src/device/cpl_native.rs` (3.6k lines; self-describes as "direct, private port") | `src/spicelib/devices/cpl/cplsetup.c`, `cpl/cplload.c` | Modified BSD — "Copyright 1992 Regents of the University of California. Author: 1992 Charles Hough" | low | Attribution added to `NOTICE`; recommend in-file header notice. |
| `crates/rspice-core/src/device/transmission_line/txl.rs` (1.1k lines, TXL convolution runtime incl. integer-picosecond clock) | `src/spicelib/devices/txl/txlload.c` (+ txl setup) | Modified BSD — same Charles Hough / UC Regents header | low | Same as above. |
| `models/veriloga/bsim4.va` (former vendored file, now removed) | Not from ngspice: UC Berkeley **BSIM4 v4.8 Verilog-A** ("Copyright 2001 Regents of the University of California", Liu/Xi/Cao/Wan/Chan/Hu), Xyce-adapted variant (`__XYCE_VAMS__` define) | Berkeley BSIM license (BSD-3-Clause in current BSIM releases); header carried only the Regents copyright block | resolved | The ambiguous root-level placeholder was removed when the shipped Verilog-A library was reorganized under `models/veriloga/cmc/`. Do not reintroduce Xyce-sourced `.va` files; use package-local upstream CMC sources instead. |
| `tests/ngspice/` (trimmed runtime corpus: 113 `.cir` decks, `.out` oracles, required include/model data, `.gates.tsv` sidecars, `validation-manifest.tsv`, `README`) | ngspice-46 `tests/` with one documented local normalization in `bsim3soifd/nmosfd.mod` (`RTH0 94 = .006` -> `RTH0 = .006`); upstream runner/build files and CMC QA-only harness trees are intentionally not shipped | Modified BSD — `COPYING` explicitly covers "test and example files" | low | Attribution added to `NOTICE`. See §4. |

### Class (b) — algorithms studied / equations translated from named ngspice files (all Modified BSD)

| RSpice file(s) | ngspice source studied | License | Risk |
|---|---|---|---|
| `device/coupled_transmission_line.rs` (mirrors `CPLload` loop structure, VI-history merging) | `cpl/cplload.c` | BSD-3 (UC Regents, Hough) | low |
| `device/transmission_line/line.rs`, `distributed.rs`, `transmission_line.rs` (LTRA interpolation modes, RLC special case, breakpoint tolerances, load split) | `ltra/ltraload.c` etc. | BSD-3 ("Copyright 1990 Regents…, Author: 1990 Jaijeet S. Roychowdhury") | low |
| `device/mosfet/jfet/mesa.rs` ("Equations are ported from ngspice HFET1"), `jfet/bias.rs` (`hfetload.c:diode`, `hfetload.c:leak`, `jfetload.c` asymptote, `DEVpnjlim`), `jfet/capacitance.rs` | `hfet1/hfetload.c`, `hfet1/hfettemp.c` ("Imported from MacSpice3f4 — Antony Wilson"; no separate license ⇒ ngspice BSD umbrella), `mesa/mesaload.c` ("Copyright 1993 Ytterdal, Lee, Shur, Fjeldly"), `jfet/jfetload.c`, `devsup.c` (UC Regents) | BSD-3 | low |
| `device/semiconductor/bjt/**` (VBIC MNA node topology per `vbicsetup.c`, gmin/pnjlim load discipline per `bjtload.c`/`vbicload.c`, substrate topology) | `vbic/vbicsetup.c`, `vbic/vbicload.c`, `bjt/bjtload.c` | BSD-3 (UC Regents; VBIC model author Colin McAndrew, Spice3 impl. Dietmar Warning) | low |
| `device/mosfet/mosfet/current.rs` (MOS1/MOS6 operating-point equations "follows the ngspice mos6load.c equations"), `construction.rs` (MOS2/MOS6 model-card defaults from `mos2set.c`) | `mos1/`, `mos2/mos2set.c`, `mos6/mos6load.c` | BSD-3 (UC Regents; MOS6 author Takayasu Sakurai) | low |
| `engine/transient.rs`, `engine/transient/truncation.rs` (divided-difference LTE / `CKTterr`-style charge truncation), `engine/transient/breakpoints.rs`, `analysis/core/transient.rs` (`dctran.c` step factors, `NIpred`, `NIiter` minimum-iteration rule) | `src/spicelib/analysis/dctran.c`, `src/maths/ni/niiter.c` etc. | BSD-3 (UC Regents, Quarles) | low |
| `circuit/storage/sources.rs`, `engine/source_values.rs`, `netlist/ast.rs` (SFFM/AM waveform formulas, defaults, MDI clamping, t<=TD behavior) | `vsrc/vsrcload.c` | BSD-3 (UC Regents, Quarles) | low |
| `engine/transient/vbic/*`, `engine/convergence/damping.rs` (pnjlim discipline), `circuit/nonlinear.rs` (CKTgmin semantics) | `devsup.c`, `vbic/*` | BSD-3 | low |

### Class (a) — behavioral parity only (no license consequence)

- `crates/rspice-core/src/testing/ngspice_runner/**` and `crates/rspice-core/tests/*` — regression harness, oracle comparison, tolerances. Original code; ngspice used as an external oracle.
- `crates/rspice-core/tests/testdata/vbic_xf_ce_ngspice46.dat` and `device/*/testdata/` replay fixtures — *output data* captured from ngspice runs (gdb-extracted histories, waveform tables). Facts/data, not ngspice code.
- `crates/rspice-cli/src/commands/run/frequency.rs` — byte-identical `.TF` output **format** matching. Format is not copyrightable expression of ngspice source; clean.
- `crates/rspice-core/src/netlist/expr/parser.rs`, `netlist/parser/conditionals.rs`, `expr/eval.rs` — **numparam semantics only** (`<>`, single-`=` equality, `.if` evaluation-point rules). Verified: original Rust implementation; no LGPL numparam code translated. Keep it that way (see §7).
- `crates/rspice-core/src/xspice/**` — implements the published **XSPICE specification** (spec/behavior level; ngspice's xspice core is public domain anyway). Verified: **no `table` code model present** (the GPLv2+ `xspice/icm/table` is the only copyleft part of xspice).
- `crates/rspice-core/src/constants.rs`, `analysis/advanced/noise.rs`, `device/thermal.rs` — REFTEMP/ITL4 etc.: numeric facts.
- `crates/rspice-bench/**`, `benchmarks/circuits/*` — original harness; decks written fresh (ring51.cir adapted from the vendored test family, which is BSD-covered).
- `docs/ROADMAP.md`, `README.md`, `design/` — descriptive references only.
- `crates/rspice-ui/src/io/waveform_io/**` — reads/writes the NUTMEG raw **file format** (format compatibility, no code derivation).
- `crates/rspice-core/src/solver/klu.rs` and its call sites in
  `solver/sparse.rs` — KLU-class sparse circuit LU implemented in original
  Rust. The source uses faer for AMD ordering; it does not vendor or bind
  SuiteSparse KLU or ngspice `src/maths/KLU`.
- `crates/rspice-ui/src/simulation/dialog/options/enums.rs` — references KLU
  only as a solver-family/UI term; no code is copied from ngspice KLU.

## 4. Vendored test decks assessment

- `tests/ngspice/` is a trimmed vendored copy of ngspice-46's runtime test materials (113 `.cir` decks plus reference `.out` files, required include/model data, `.gates.tsv` sidecars, `validation-manifest.tsv`, and ngspice's own `tests/README`) with one documented model-card normalization in `bsim3soifd/nmosfd.mod`: `RTH0 94 = .006` is corrected to `RTH0 = .006`, matching the sibling BSIM3SOI model cards. Upstream runner/build files (`bin/`, `Makefile.am`, `ChangeLog`) and CMC QA-only harness trees (`qaSpec`, `parameters`, `.standard` references without RSpice-discoverable `.cir` decks) are intentionally not shipped. The recorded `.out` oracles are unchanged. Some decks originate from Spice3f5 and MacSpice3f4 per their comment headers; all are distributed by ngspice under its blanket Modified BSD grant ("source code, test and example files").
- **Redistribution in a commercial/proprietary repo is permitted** under BSD-3 provided the copyright notice, condition list, and disclaimer are reproduced — now done in the repository `NOTICE`.
- **Xyce_Regression (GPL-3.0-or-later):** `tests/xyce/` vendors the runtime regression materials from `C:\Users\James\Desktop\Xyce_Regression-master`: `Netlists/`, `OutputData/`, upstream `README.md`, `COPYING`, `RSPICE-VENDORING.md`, and `RSPICE-HARNESS-MANIFEST.tsv`. Upstream platform scripts (`TestScripts/`, `.cir.sh`, Perl/Python helpers), tag/exclude selection files, per-directory runner manifests, and CMake/CTest configuration files are intentionally omitted because RSpice runs this corpus through its Rust-native adapter. The local checkout's README referenced `COPYING` but no `COPYING` file was present locally, so RSpice added GPL-3.0 text at `tests/xyce/COPYING`. GPL terms apply to redistribution of this corpus.
- The CMC-style QA scripts from ngspice's upstream `tests/` tree (`bin/ngspice.pm`, `run_cmc_check`, `qaSpec` files) carry no separate license header and fall under the same blanket BSD statement, but they are not included in the trimmed RSpice corpus because the Rust harness does not use them.

## 5. Embedded assets

- **IBM Plex fonts** (`crates/rspice-ui/assets/fonts/`: Sans Regular/Medium/SemiBold, Mono Regular/Medium) — SIL OFL 1.1. License file **confirmed present**: `crates/rspice-ui/assets/fonts/OFL-IBMPlex.txt` ("Copyright © 2017 IBM Corp. with Reserved Font Name 'Plex'"). OFL obligations (keep notice with the fonts, no selling fonts standalone) are met; also listed in `NOTICE`.
- `assets/` (logo, images) — project-authored.
- `models/spice/*.lib` — RSpice-authored model libraries (headers say "RSpice … Library"). `models/veriloga/constants.vams` / `disciplines.vams` — written against IEEE 1800/Verilog-A LRM definitions (facts/standard names). `models/veriloga/cmc/` — shipped CMC packages retain their package-local license and notice files.

## 6. Dependency licenses (direct deps of workspace crates; license fields verified from cargo registry cache)

Everything is permissive. Non-(pure MIT/Apache) entries worth tracking:

| Crate | License | Note |
|---|---|---|
| `usvg` 0.44 (rspice-ui) | **MPL-2.0** | File-level copyleft. Fine as an unmodified dependency; if RSpice ever patches usvg source, those files must stay MPL and be made available. Listed in NOTICE. |
| `numpy` 0.23 (rspice-python) | BSD-2-Clause | Attribution in NOTICE. |
| `libloading` (rspice-core, optional) | ISC | Permissive. |
| `winit` 0.30 (rspice-ui) | Apache-2.0 (only) | Apache NOTICE-preservation applies (winit ships no NOTICE file). |
| `wide`, `bytemuck`, `raw-window-handle` | Zlib OR Apache-2.0 OR MIT | MIT option exercised. |
| `blake3` (optional) | CC0-1.0 OR Apache-2.0 (OR Apache-2.0 WITH LLVM-exception) | Permissive. |
| `pollster` | Apache-2.0/MIT | dual. |
| `faer`, `nom`, `bincode`, `tokio`, `indicatif`, `mimalloc`, `rfd`, `rustyhdf5`, `serde-wasm-bindgen` | MIT | — |
| everything else (`serde`, `clap`, `egui`/`eframe`/`egui-wgpu`, `wgpu`, `rustfft`, `rayon`, `thiserror`, `log`, `png`, `base64`, `uuid`, `once_cell`, `indexmap`, `smol_str`, `pyo3`, `toml`, `dirs`, `getrandom`, `wasm-bindgen`, `svgtypes`, …) | MIT OR Apache-2.0 | — |

No LGPL/GPL crates in the dependency graph's direct tier. (`rustyhdf5` confirmed `license = "MIT"` in its published manifest.)

## 7. What to avoid going forward

1. **Never port, translate, or "study-then-rewrite" code from these ngspice directories** (copyleft): `src/maths/KLU` (LGPLv2), `src/frontend/numparam` (LGPLv2+), `src/xspice/icm/table` (GPLv2+), `src/tclspice.c` (LGPLv2), `src/osdi` (MPL-2.0). Matching their *behavior* from documentation, black-box testing, or the (CC-BY-SA) manual's described semantics is fine — that is exactly how the `.if/.elseif` numparam-semantics work was done; keep that discipline.
2. **Solver work:** continue implementing KLU-class behavior from published papers (Davis/Palamadai Natarajan, AMD/BTF literature), black-box behavior, and permissively licensed Rust libraries (`faer`). Do **not** read KLU source as a reference. If a KLU binding is ever truly needed, link SuiteSparse KLU dynamically and respect LGPL terms — keep it an optional, clearly isolated feature and record a new audit addendum.
3. **A `table` code model** for xspice compatibility must be written from the ngspice manual's description only — the reference implementation is GPLv2+.
4. When porting any further ngspice device (BSD-covered dirs), check the file header first, and carry the upstream copyright line into the Rust module header at port time, plus a `NOTICE` entry.
5. Keep Xyce_Regression GPL-3.0-or-later materials isolated under `tests/xyce/` with `COPYING`, upstream README, `RSPICE-VENDORING.md`, and the RSpice harness manifest; do not mix Xyce scripts or generated model sources into RSpice source modules. If BSIM4 Verilog-A is reintroduced, use only a pristine official upstream source and record provenance in this audit and `NOTICE`.
6. The ngspice **manual** is CC-BY-SA 4.0: don't paste manual text into RSpice docs (share-alike would attach); paraphrase.
7. ngspice oracle *outputs* (waveform tables, gdb-extracted fixtures) are safe to check in; keep labeling them with the generating version as is current practice.

## 8. Conclusion

All identified code-level derivation (class b/c) traces to permissive upstream sources: Modified-BSD-covered ngspice files and the UC Berkeley BSIM4 ECL-2.0 / B4TERMS_OF_USE source set. The obligations are: reproduce the applicable copyright notices, conditions, and disclaimers (tracked in `NOTICE`), keep required BSIM Research Group acknowledgement in product documentation, comply with applicable export restrictions, and do not use UC/contributor names for endorsement. Shipped CMC Verilog-A packages retain their package-local license and notice files under `models/veriloga/cmc/`. No LGPL/GPL ngspice code was found in the tree; the in-tree KLU-class solver is original Rust and no SuiteSparse KLU source or binding is vendored. Open follow-up: add upstream copyright lines to the directly-ported module headers (`b3soi/**`, `cpl_native.rs`, `txl.rs`).
