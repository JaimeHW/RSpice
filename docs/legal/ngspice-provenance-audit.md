# ngspice Provenance Audit

**Date:** 2026-06-10
**Scope:** RSpice repository (`C:\Users\James\Desktop\RSpice`) audited against the local ngspice-46 source tree (`C:\Users\James\Desktop\ngspice-46-release\ngspice-46`).
**Status:** Engineering research, not legal advice. Prepared by automated/manual code survey; have counsel review before relying on it for licensing or distribution decisions.

---

## 1. Methodology

1. Case-insensitive content search of the entire RSpice tree for: `ngspice`, `vsrcload`, `spice3`, `ported from`, `adapted from`, `based on`, `translated from`, `transcribed`, `faithful port`, `direct port`, `numparam`, `xspice`, `KLU`, `berkeley` (996 occurrences across 200+ files; `target/` excluded via gitignore).
2. Every hit in `crates/`, `models/`, `benchmarks/`, `docs/`, and `tests/` was classified:
   - **(a) behavioral parity** — test oracle, output-format matching, semantics matching, constants that are facts (defaults, tolerances). No license consequence.
   - **(b) algorithm studied / translated at equation level** from a named ngspice file. The named file's license header was read in the local ngspice-46 tree.
   - **(c) direct port / verbatim copy** of code or files. License of the source file attaches.
3. ngspice's `COPYING` (root of ngspice-46) was read in full to map its mixed-license structure.
4. Vendored assets checked: `tests/` deck tree, `models/`, `assets/`, embedded fonts.
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

**Key audit result: every ngspice file that RSpice ported from or studied is in the Modified-BSD-covered portion. Nothing was taken from KLU, numparam, xspice/icm/table, tclspice, osdi, or the manual.**

## 3. Findings table

Risk legend: **none** = behavioral only; **low** = BSD-3 attribution required (now provided via `NOTICE`); **flag** = needs follow-up.

### Class (c) — direct ports / verbatim copies (BSD-3 attribution required)

| RSpice file(s) | ngspice source | License (header checked) | Risk | Action |
|---|---|---|---|---|
| `crates/rspice-core/src/device/mosfet/b3soi/**` (~13.9k lines: `dd/`, `fd/`, `pd/` — `eval.rs` "faithful port", `temp.rs` "faithful port", `params.rs` "transcribed", `common.rs` constants) | `src/spicelib/devices/bsim3soi_dd/`, `bsim3soi_fd/`, `bsim3soi_pd/` (`b3soi{dd,fd,pd}ld.c`, `*temp.c`, `*set.c`, `*.c` param tables) | Modified BSD — "Copyright 1990/1999 Regents of the University of California", authors Fung/Sinitsky/Tang/Liu/Su et al. | low | Attribution added to `NOTICE`. Recommended: add the UC Regents copyright line to the module headers of the ported files. |
| `crates/rspice-core/src/device/cpl_native.rs` (3.6k lines; self-describes as "direct, private port") | `src/spicelib/devices/cpl/cplsetup.c`, `cpl/cplload.c` | Modified BSD — "Copyright 1992 Regents of the University of California. Author: 1992 Charles Hough" | low | Attribution added to `NOTICE`; recommend in-file header notice. |
| `crates/rspice-core/src/device/transmission_line/txl.rs` (1.1k lines, TXL convolution runtime incl. integer-picosecond clock) | `src/spicelib/devices/txl/txlload.c` (+ txl setup) | Modified BSD — same Charles Hough / UC Regents header | low | Same as above. |
| `models/veriloga/bsim4.va` (verbatim vendored file) | Not from ngspice: UC Berkeley **BSIM4 v4.8 Verilog-A** ("Copyright 2001 Regents of the University of California", Liu/Xi/Cao/Wan/Chan/Hu), Xyce-adapted variant (`__XYCE_VAMS__` define) | Berkeley BSIM license (BSD-3-Clause in current BSIM releases); header carries only the Regents copyright block | **flag (low-medium)** | Attribution added to `NOTICE`. **Confirm origin**: the `__XYCE_VAMS__` marker suggests it was obtained via the Xyce project (GPL-3 repo). The file itself is Berkeley-copyright model code that Xyce redistributes, not Xyce-authored code, but the cleanest fix is to re-vendor the pristine `.va` from the official Berkeley BSIM4.8 release and document that origin. |
| `tests/` (entire tree: 113 `.cir` decks, `.out` oracles, `bin/` scripts incl. `ngspice.pm`, `check.sh`, CMC `qaSpec` harness, `Makefile.am`s, `README`, `ChangeLog`) | ngspice-46 `tests/` (verbatim copy) | Modified BSD — `COPYING` explicitly covers "test and example files" | low | Attribution added to `NOTICE`. See §4. |

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
- `crates/rspice-ui/src/simulation/dialog/options/enums.rs` — mentions KLU only as a *named option in ngspice* for UI parity; **no KLU code or binding exists in the tree** (`faer` is the solver).

## 4. Vendored test decks assessment

- `tests/` is a verbatim copy of ngspice-46's `tests/` directory (113 `.cir` decks plus reference `.out` files, the Perl/shell QA harness in `tests/bin/`, `Makefile.am`s, and ngspice's own `tests/README`). Some decks originate from Spice3f5 and MacSpice3f4 per their comment headers; all are distributed by ngspice under its blanket Modified BSD grant ("source code, test and example files").
- **Redistribution in a commercial/proprietary repo is permitted** under BSD-3 provided the copyright notice, condition list, and disclaimer are reproduced — now done in the repository `NOTICE`.
- **Xyce_Regression (GPL-3):** verified that nothing from it is vendored — `grep -ril xyce tests/` returns no hits. Project policy (CI-clone-only) is being followed for `tests/`. The one Xyce-marked file in the repo is `models/veriloga/bsim4.va` (Berkeley-copyright model code, see §3 class (c)) — re-vendor from the Berkeley BSIM release to remove any ambiguity.
- The CMC-style QA scripts (`tests/bin/ngspice.pm`, `run_cmc_check`, `qaSpec` files) ship inside ngspice's `tests/` and carry no separate license header; they fall under the same blanket BSD statement.

## 5. Embedded assets

- **IBM Plex fonts** (`crates/rspice-ui/assets/fonts/`: Sans Regular/Medium/SemiBold, Mono Regular/Medium) — SIL OFL 1.1. License file **confirmed present**: `crates/rspice-ui/assets/fonts/OFL-IBMPlex.txt` ("Copyright © 2017 IBM Corp. with Reserved Font Name 'Plex'"). OFL obligations (keep notice with the fonts, no selling fonts standalone) are met; also listed in `NOTICE`.
- `assets/` (logo, images) — project-authored.
- `models/spice/*.lib` — RSpice-authored model libraries (headers say "RSpice … Library"). `models/veriloga/constants.vams` / `disciplines.vams` — written against IEEE 1800/Verilog-A LRM definitions (facts/standard names). `models/veriloga/bsim4.va` — see §3.

## 6. Dependency licenses (direct deps of workspace crates; license fields verified from cargo registry cache)

Everything is permissive. Non-(pure MIT/Apache) entries worth tracking:

| Crate | License | Note |
|---|---|---|
| `usvg` 0.44 (rspice-ui) | **MPL-2.0** | File-level copyleft. Fine as an unmodified dependency; if RSpice ever patches usvg source, those files must stay MPL and be made available. Listed in NOTICE. |
| `numpy` 0.23 (rspice-python) | BSD-2-Clause | Attribution in NOTICE. |
| `libloading` (rspice-core, optional) | ISC | Permissive. |
| `winit` 0.30 (rspice-ui) | Apache-2.0 (only) | Apache NOTICE-preservation applies (winit ships no NOTICE file). |
| `cranelift*` 0.115 (rspice-veriloga, optional) | Apache-2.0 WITH LLVM-exception | Permissive. |
| `wide`, `bytemuck`, `raw-window-handle` | Zlib OR Apache-2.0 OR MIT | MIT option exercised. |
| `blake3` (optional) | CC0-1.0 OR Apache-2.0 (OR Apache-2.0 WITH LLVM-exception) | Permissive. |
| `pollster` | Apache-2.0/MIT | dual. |
| `faer`, `nom`, `bincode`, `tokio`, `indicatif`, `mimalloc`, `rfd`, `rustyhdf5`, `serde-wasm-bindgen` | MIT | — |
| everything else (`serde`, `clap`, `egui`/`eframe`/`egui-wgpu`, `wgpu`, `rustfft`, `rayon`, `thiserror`, `log`, `png`, `base64`, `uuid`, `once_cell`, `indexmap`, `smol_str`, `pyo3`, `toml`, `dirs`, `getrandom`, `wasm-bindgen`, `svgtypes`, …) | MIT OR Apache-2.0 | — |

No LGPL/GPL crates in the dependency graph's direct tier. (`rustyhdf5` confirmed `license = "MIT"` in its published manifest.)

## 7. What to avoid going forward

1. **Never port, translate, or "study-then-rewrite" code from these ngspice directories** (copyleft): `src/maths/KLU` (LGPLv2), `src/frontend/numparam` (LGPLv2+), `src/xspice/icm/table` (GPLv2+), `src/tclspice.c` (LGPLv2), `src/osdi` (MPL-2.0). Matching their *behavior* from documentation, black-box testing, or the (CC-BY-SA) manual's described semantics is fine — that is exactly how the `.if/.elseif` numparam-semantics work was done; keep that discipline.
2. **Solver work:** when building the KLU-class solver (ROADMAP WS: BTF + AMD + left-looking LU), implement from the published papers (Davis/Palamadai Natarajan, AMD/BTF literature) or use/keep pure-Rust libraries (`faer`). Do **not** read KLU source as a reference. If a KLU binding is ever truly needed, link SuiteSparse KLU dynamically and respect LGPL terms — keep it an optional, clearly isolated feature.
3. **A `table` code model** for xspice compatibility must be written from the ngspice manual's description only — the reference implementation is GPLv2+.
4. When porting any further ngspice device (BSD-covered dirs), check the file header first, and carry the upstream copyright line into the Rust module header at port time, plus a `NOTICE` entry.
5. Keep Xyce and Xyce_Regression (GPL-3) CI-clone-only; never vendor decks, scripts, or `.va` files from those repos. Replace `models/veriloga/bsim4.va` with the pristine Berkeley release copy.
6. The ngspice **manual** is CC-BY-SA 4.0: don't paste manual text into RSpice docs (share-alike would attach); paraphrase.
7. ngspice oracle *outputs* (waveform tables, gdb-extracted fixtures) are safe to check in; keep labeling them with the generating version as is current practice.

## 8. Conclusion

All identified code-level derivation (class b/c) traces to Modified-BSD-covered ngspice files — predominantly UC Regents copyright. The obligations are: reproduce the copyright notice, conditions, and disclaimer (done in `NOTICE`), and do not use UC/contributor names for endorsement. No LGPL/GPL ngspice code was found in the tree. Open follow-ups: (1) re-vendor `models/veriloga/bsim4.va` from the official Berkeley BSIM4.8 distribution; (2) add upstream copyright lines to the directly-ported module headers (`b3soi/**`, `cpl_native.rs`, `txl.rs`).
