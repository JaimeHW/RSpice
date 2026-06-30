# RSpice World-Class Program

Governing engineering plan to take RSpice from "validated ngspice-class core" to a simulator that is
strictly better than every free alternative (ngspice, LTspice, Xyce, QSPICE) and credibly competes
with Spectre/HSPICE. Business/commercial strategy is intentionally out of scope here; this is the
engineering program.

Status note (2026-06-16): this file is a forward engineering program and
backlog, not the current product contract. Since the first draft, several
items have landed or partially landed: `.tf`, `.save`/`.probe`/`.print`
parsing and output selection, SFFM/AM source functions, TRNOISE white + 1/f
noise, a CLI S-parameter path over `.ac`, HSPICE-style `.alter`/`.data`, and
the default KLU-class real linear-solver backend.

Status note (2026-06-25): canonical Verilog-A IR and the first build-time
Verilog-A-to-Rust backend are now the default product path through
`veriloga-builtins`, with a generated registry materialized under
`crates/rspice-core/src/device/`. Remaining bullets below should be read as
qualification, corpus coverage, performance gates, artifact-contract hardening,
and compatibility closure unless they explicitly describe an unimplemented
surface.

Ground rules (carried over from the regression program, non-negotiable):

- **Fix the simulator, never loosen the harness.** Accuracy gates only move with adjudicated evidence.
- **Modern/professional/accurate wins** when a reference contradicts a current official binary.
- **No knob-tuning toward historical artifacts.** Grid-parity is achieved by faithful mechanisms or
  by grid-locked comparison infrastructure, never by tweaking trajectories to match stale tables.
- **Every claim is backed by a checked-in, repeatable test.** No "works on my machine" features.

---

## Program structure

| WS | Name | One-line goal | Depends on |
|----|------|---------------|------------|
| WS0 | Validation & benchmark foundations | Every later claim measurable; corpus + oracles + perf rigs in place | — |
| WS1 | Netlist ingestion & dialect compatibility | Real-world vendor models and PDK decks parse and run | WS0 corpus |
| WS2 | Compact-model program (Verilog-A to generated Rust) | Industry CMC models run as generated native Rust from upstream Verilog-A | WS0, parts of WS1 |
| WS3 | Performance & scale | Beat ngspice single-thread; scale to 100k+ nodes; parallel sweeps | WS0 benchmarks |
| WS4 | RF validation & completion | PSS/HB/PAC/PNoise proven against oracles and analytic truth | WS0; WS2 for HBT decks |
| WS5 | Mixed-signal completion | XSPICE digital event engine finished and validated (or formally descoped) | WS0 |
| WS6 | Product, content & documentation | UI polish, .asc import, model library, user manual, installers | independent |
| WS7 | Commercial qualification | Reproducibility, release engineering, encrypted models, support surface | all |

WS0 is the unlock. WS1–WS3 are the adoption-critical core and can run concurrently (disjoint code
lanes suit the multi-session dev model). WS4 is the differentiation wedge. WS5–WS7 close out
"world-class by all metrics."

---

## WS0 — Validation & benchmark foundations

**Why first:** the gap analysis found zero performance evidence, no RF regression decks, and 8/113
legacy decks blocked on reference-provenance policy. Every other workstream needs corpus + oracle +
measurement infrastructure to define "done."

### M0.1 — Reference modernization policy (closes the legacy 8)
- Implement **grid-locked oracle comparison**: run RSpice on the reference's recorded time grid
  (forced external timestep schedule) and compare values at equal grids. This separates physics
  parity from dt-sequence parity — the Spectre-grade resolution to the mosamp/schmitt/transmission
  family. (All transmission runtimes are already proven bit-faithful by replay fixtures.)
- Regenerate provably-stale references from the official ngspice-46 binary under the established
  res_array precedent (old ref fails AND ngspice-46 cannot reproduce it).
- Exit: suite policy documented in README; scoreboard target **113/113** under the documented
  policy (pointwise where honest, grid-locked or regenerated where adjudicated).

### M0.2 — Corpus expansion from already-local material
- Mine `C:\Users\James\Desktop\ngspice-46-release\ngspice-46\examples\` (**462 decks**, categories
  absent from the vendored 113): `Monte_Carlo/`, `pss/`, `sp/` (S-parameter), `soa/`,
  `transient-noise/`, `klu/` (large matrices — perf seeds), `SkywaterOpenSourcePDK/`, `digital/`,
  `xspice/`, `hicum2/`, `memristor/`, `vdmos/`, `soi/`, `numparam/`, `measure/`, `control_structs/`.
- Triage each category: runnable-now → add to harness with live-ngspice-generated references;
  feature-gap → file as acceptance test for the owning workstream (e.g. `sp/` → WS4, `digital/` →
  WS5, `Monte_Carlo/` (uses agauss) → WS1).
- License: same ngspice licensing as the already-vendored `tests/ngspice/` tree (BSD-style core) — keep
  upstream notice files alongside vendored decks.

### M0.3 — External public corpora (the user-facing validation answer)
| Corpus | What it validates | License | Integration mode |
|---|---|---|---|
| SkyWater **sky130** PDK | Binned BSIM4 cards (63 bins/device), 7 corners, AGAUSS MC mismatch, real foundry decks | Apache-2.0 | Vendor a pinned subset (cards + smoke circuits); full PDK fetched in CI |
| **GF180MCU** PDK | Second independent foundry BSIM4 card set (180 nm) | Apache-2.0 | Same pattern |
| **IHP SG13G2** PDK | **PSP 103.6** MOSFETs + **HICUM/L2** SiGe HBTs (350 GHz fT) — the system test for WS2 and WS4 RF | Apache-2.0 | Same pattern |
| **Xyce_Regression** (Sandia) | 2000+ netlists with gold outputs; HB/AC/noise/devices breadth | GPL-3 | Vendor under `tests/xyce/` with upstream README, `COPYING`, and `RSPICE-VENDORING.md`; keep isolated from product/runtime code and run through a Xyce-specific adapter |
| Sandia **XDM** converter | Authoritative PSPICE/HSPICE dialect semantics (reference, not dependency) | GPL-3 | Read-only reference for WS1 dialect tables |
| CMC standard models (Si2): BSIM-CMG v112, BSIM-IMG, BSIM-BULK, PSP, HICUM L0/L2, MEXTRAM, ASM-HEMT, MVSG | Verilog-A sources; private official QA decks require CMC membership and are not a planned gate without access | Public download (form); package-specific redistribution terms | Shipped, redistributable Verilog-A packages live under `models/veriloga/cmc/`; OMI remains excluded pending separate review |
| Berkeley SPICE3f5 test decks | Classic core-engine decks | UC Berkeley | Vendor |
| **scikit-rf** test files | Touchstone parser/export round-trip corpus | BSD-3 | Vendor subset |
| Vendor SPICE models (TI, ADI, Infineon, onsemi, Nexperia, Coilcraft) | Real-world ingestion (PSpice/LTspice/HSPICE dialects) | Free download, **not redistributable** | `tools/fetch-compat-corpus` script + private cached mirror; CI-only, never vendored |
| In-house analytic decks | Closed-form truth independent of any simulator: RLC step/impulse, Butterworth/Chebyshev responses, ideal-mixer conversion matrices, Leeson-model oscillators, telegrapher solutions | Ours | Vendor; these are the only simulator-independent oracles — grow continuously |

### M0.4 — Oracle tiers in the harness
- Tier 1: checked-in references (exists). Tier 2: live ngspice (exists). Add **Tier 3: live Xyce**
  (`RSPICE_XYCE_LIVE_REFERENCES`) — the only free HB/RF cross-oracle. Add **Tier 0: analytic**
  (exact expressions evaluated in-harness; strictest tolerances).

### M0.5 — Performance benchmark rig (gates WS3)
- `benches/` with criterion micro-benches (stamp, factor, solve, device eval) plus a macro-bench
  binary: standard circuit set (ring oscillators 51/101/501-stage in 3 technologies, sky130
  op-amp tran/AC/noise, buck converter, 10k/100k-node post-layout RC ladder + clock tree, MC×500 on
  an OTA, `klu/` example matrices). Runs RSpice vs local ngspice (vs Xyce optional), emits a JSON
  scoreboard; CI tracks regressions >5%.
- Exit: first published scoreboard checked into `diagnostics/benchmarks/` — whatever it shows.

---

## WS1 — Netlist ingestion & dialect compatibility

**Goal:** an evaluator can drop in a TI/ADI PSpice macromodel, an Infineon power MOSFET .lib, an
LTspice deck, or a foundry HSPICE PDK and it parses and runs. This is the #1 adoption blocker.

### M1.1 — Core gaps in the native (ngspice) dialect
- `.if/.elseif/.else/.endif` preprocessor conditionals (sky130 cards use them).
- Inline statistical functions in the expression engine: `gauss/agauss/unif/aunif/flat/limit`,
  wired to per-run RNG streams so `.MC` and sky130 `MC_MM_SWITCH` mismatch work end-to-end.
- Source-function closure: `SFFM`, `AM`, and `TRNOISE` white + 1/f are implemented; remaining
  compatibility work is `TRRANDOM`, RTS-tail `TRNOISE`, and corpus parity against ngspice's
  `examples/transient-noise`.
- Front-door wiring for existing analyses: `.tf` and `.save`/`.probe`/`.print` are implemented;
  remaining compatibility work is the `.sp` dot-command and broader S-parameter/noise-matrix
  integration beyond the current CLI `--sparam` path over `.ac`.
- `.option savecurrents`, SOA checks (`.option warn`, device SOA limits — ngspice `soa/` decks
  as acceptance).
- E/F/G/H source extensions: E/G support linear, `POLY(n)`, `VALUE={}`, `TABLE`, and `LAPLACE`;
  F/H support linear and `POLY(n)`. Remaining PSpice-compatibility work is `FREQ` and any
  vendor-macromodel variants surfaced by the compatibility corpus.
- Exit gate: ngspice `examples/Monte_Carlo`, `soa`, `transient-noise`, `numparam`, `measure`
  categories green in the harness.

### M1.2 — Dialect compatibility layer
- Architecture: a **dialect mode** on the parser (`ngspice | pspice | ltspice | hspice`, auto-detected
  from file extension/content heuristics, overridable), implemented as token-level shims + element
  translation tables, not forked parsers. Use Sandia XDM's translation tables as the semantic
  reference for PSPICE/HSPICE quirks.
- PSpice: `.PROBE`, `GAIN/SUM/MULT` ABM blocks, `.PARAM` distribution syntax, model `AKO:`,
  `.FUNC` differences, pin-order quirks, `{}` everywhere, `.STEP PARAM` variants.
- LTspice: `.step oct/param` forms, behavioral `Rser/Cpar` on passives, `V=F(...)` B-source
  spellings, `.func`, `.wave`, `SW` model levels, `.backanno`, undocumented-but-ubiquitous idioms
  (charge-based behavioral caps `Q=`).
- HSPICE: `.alter`, `.data`/`.enddata` sweeps, `.measure` with sweep context, `$`/`$$` comments,
  `.protect/.unprotect`, vectorized sweeps, `M=`/`S=` scale options, `.option` aliases. HSPICE is
  the PDK lingua franca — prioritize the subset foundry decks actually use.
- Exit gate: **compat corpus scoreboard** — top-20 most-downloaded vendor models per vendor
  category (op-amp, power FET, LDO, comparator, inductor) parse, run their vendor-supplied test
  fixture, and match LTspice/ngspice-reference waveforms within tolerance. Scoreboard checked in;
  CI runs from the cached mirror.

### M1.3 — Scripting & automation story
- Decision point: implement a `.control`-equivalent batch scripting subset (loops, `alter`,
  `let`, `meas`, plot-to-file) **or** declare Python the official scripting surface and ship
  `rspice run --script foo.py` with a stable in-process API. Recommendation: the latter (it is
  already production-grade) + a small `.control` compatibility subset for ngspice deck reuse
  (enough to run `examples/control_structs`).

---

## WS2 — Compact-model program (Verilog-A to generated Rust)

**Goal:** close the modern-silicon model gap (FinFET/FDSOI/PSP/HICUM/MEXTRAM/GaN) without
hand-porting CMC compact models. CMC Verilog-A packages under `models/veriloga/cmc/` are the source
of truth; the strategic deliverable is a Verilog-A to Rust transpiler that emits native Rust device
implementations from those sources.

### M2.1 — VA pipeline industrial qualification
- Target: compile shipped CMC Verilog-A packages under **`models/veriloga/cmc/`** end-to-end,
  beginning with BSIM-CMG, BSIM-BULK, PSP, HICUM/L2, and Diode CMC, then generate Rust devices
  that match interpreted Verilog-A and cross-oracle results where available.
- Policy: no new hand-native CMC model ports. Historical hand-native CMC slices are reference
  material only; generated Rust from the Verilog-A source is the active implementation path.
- Required runtime features to audit/complete: `ddx`, `$limit` with pnjlim-equivalent callbacks
  (Newton limiting parity), internal-node collapse, `white_noise/flicker_noise` → noise-analysis
  integration, parameter ranges/defaults, `$param_given`, temperature update path, bias-independent
  precomputation split (setup vs load separation — critical for speed).
- Generated-Rust performance gate: ≤2× hand-native BSIM4 eval time for comparable compact-model
  kernels, with interpreted VM retained as a debugging/reference fallback.
- Deliverable: `rspice compile-va` can emit or cache a generated Rust artifact loadable like a
  built-in model (OSDI-equivalent contract, documented).

### M2.2 — CMC standard models, in priority order
1. **PSP 103/104** (IHP PDK MOSFETs validate it; analog/RF foundries use it) — generated Rust
   from CMC Verilog-A, with QA decks from Si2 where available.
2. **HICUM/L2** (IHP SiGe HBTs validate it; unlocks the vendored-but-dormant `tests/ngspice/hicum2`) —
   generated Rust from CMC/Xyce Verilog-A sources, not a new hand-native port.
3. **BSIM-CMG v112** (FinFET — table stakes for "advanced-node" claims; QA decks from Si2) —
   generated Rust from `models/veriloga/cmc/BSIM-CMG_112.1.0_04282026`.
4. **BSIM-IMG** (FDSOI), **BSIM-BULK** (replaces aging BSIM4 lineage), **MEXTRAM 505**,
   **ASM-HEMT / MVSG** (GaN CMC qualification beyond the in-tree physics-style model) — generated
   Rust from the shipped CMC Verilog-A packages.
5. **HiSIM2/HiSIM-HV** via VA if sources obtainable; else native port later (unlocks vendored
   `tests/ngspice/hisim*`).
- Each model lands with: generated Rust source/artifact provenance, CMC QA deck run,
  gm/gds/caps continuity sweep tests (C∞ checks),
  ngspice/Xyce cross-oracle where they implement it, and activation of the corresponding dormant
  vendored test directory.

### M2.3 — Hand-native completion only where the VA/transpiler route does not cover
- BSIM4 native: stress effects (SA/SB/SD), `mtrlMod=1`, `rgateMod`/`rbodyMod` networks,
  `trnqsMod` NQS, `mobMod` variants — required for foundry cards that set them (sky130 sets
  several). Acceptance: full sky130 + GF180 card sets load with **zero ignored-parameter warnings**
  and corner/MC decks match ngspice.
- Model-card binning at `.model` level (harness exists in lib_parser; ensure engine selection +
  `LMIN/LMAX/WMIN/WMAX` semantics match HSPICE/ngspice including `scale`).
- Diode: reverse recovery (soft-recovery charge model), tunneling, full temperature set.
- GaN HEMT: qualify the in-tree physics-style model, then add ASM-HEMT/MVSG via VA before making
  CMC-grade GaN claims.
- Self-heating audit: VBIC thermal node exists; verify BSIM4 SHMOD and B3SOI self-heating paths or
  mark unsupported explicitly.

---

## WS3 — Performance & scale

**Goal:** measured, repeatable superiority: ≥ ngspice-KLU single-thread on the benchmark set,
near-linear multicore scaling on statistical workloads, 100k+-node capacity. No optimization
before M0.5 baselines exist.

### M3.1 — Solver core
- A **KLU-class real circuit solver** has landed and is the default backend. Remaining work is
  production-scale qualification: BTF/ordering audits, pivot-growth diagnostics, refactorization
  fallbacks, iterative refinement, and benchmark comparison against SuiteSparse KLU/ngspice-KLU.
  Keep faer as the dense/eigen and complex-solve backend where it is still the right tool.
- Complex-valued direct path for AC/HB (avoid 2N-real expansion where it costs).
- Iterative refinement + condition/pivot-growth diagnostics surfaced in `--verbose` and logs.

### M3.2 — Parallel inner loop
- Parallel device evaluation: partition by device class into thread-local stamp buffers merged
  per-iteration (the existing `AtomicMatrix` is the fallback; buffered merge avoids the 10×
  atomic penalty). Gate: ≥3× eval speedup on 8 cores for the 100k-device bench, bit-identical
  residuals (deterministic reduction order — commercial-grade reproducibility requirement).
- Bypass: default-on with ngspice-parity guards (already proven for B3SOI; generalize to BSIM4/
  BJT/diode), plus convergence-safe re-enable rules.
- Matrix assembly: per-instance precomputed CSC slot indices everywhere (audit; some paths still
  hash).

### M3.3 — Embarrassingly parallel drivers
- `.STEP` × corners × Monte Carlo × multi-`.temp`: a work-stealing multi-run executor (rayon) with
  shared parse/setup, per-run RNG streams (counter-based, run-index keyed — reproducible regardless
  of thread schedule), streaming result aggregation. Gate: ≥6× on 8 cores for MC×500 OTA bench.
- Memory: waveform streaming to disk for long tran (exists for raw export — make it the default
  path above a threshold; bound history buffers).

### M3.4 — Scale qualification
- Generated post-layout-style benches: RC ladders/meshes 10k → 1M elements; clock tree with 50k
  sinks; gate: 100k-node tran completes within 4× ngspice-KLU time and bounded memory; document
  the 1M-element behavior honestly.
- Large-circuit robustness: node-count-independent diagnostics, singular-matrix forensics
  (named nodes, suspect devices — the diffamp experience productized).

---

## WS4 — RF validation & completion

**Goal:** the RF suite (PSS/HB/PAC/PNoise/PSTB/envelope) goes from "implemented, unvalidated" to
"proven" — it is the headline differentiator over every free tool.

### M4.1 — Analytic truth tier (Tier 0)
- LPTV closed-form decks: switched-RC mixer conversion matrix, ideal commutating mixer gain
  (2/π), parametric amplifier Manley–Rowe checks, Duffing/Van der Pol limit cycles (period +
  amplitude vs perturbation series), Leeson-model oscillator phase noise slopes (-20/-30 dB/dec
  regions, corner placement), Adler injection locking range.
- Gate: each RF analysis has ≥5 analytic decks with documented closed-form derivations in-repo.

### M4.2 — Cross-oracle tier
- Xyce HB as the free cross-oracle (Tier 3): diode rectifier, BJT mixer, PA compression (P1dB,
  IIP3 two-tone), oscillator fundamental via HB. QucsStudio/Qucs-S spot checks where applicable.
- Published-circuit tier: textbook circuits with published measurements (Razavi/Lee/Hajimiri
  examples; IHP SG13G2 HBT LNA/VCO app-note circuits once HICUM lands) — tolerance bands documented
  per source quality.

### M4.3 — Completion items surfaced by validation
- Autonomous (oscillator) PSS hardening: frequency-as-unknown formulation, phase condition
  selection, robust initial-guess pipeline (tran ring-up + period detect exists — qualify it).
- PNoise: cyclostationary noise folding correctness (validate against analytic switched-noise
  decks); noise figure for mixers (SSB/DSB).
- Deck-card `.sp` front door plus noise-correlation matrices -> NF from S + noise params. The CLI
  already has two-port `--sparam` extraction over `.ac`; LSSP (large-signal S-params via HB) remains
  the Spectre-parity stretch.
- HB robustness program: continuation (source stepping in power), Krylov preconditioner
  qualification on >1k-harmonic problems, multi-tone diamond truncation documentation.
- Gate: RF regression suite ≥40 decks green in CI; RF chapter in the user manual with validated
  example gallery.

---

## WS5 — Mixed-signal completion

**Recommendation: finish it** (bounded scope) — ngspice parity claims ring hollow without XSPICE,
the vendored `tests/ngspice/xspice` + `examples/xspice|digital` corpora already exist as acceptance tests,
and power/system users expect behavioral digital.

- M5.1: event-driven core: typed event queues, delays, inertial/transport semantics, hybrid
  analog/event scheduler integration with the transient loop (breakpoint injection on events).
- M5.2: the 12 workhorse code models: d_dff/d_latch/gates/d_state, adc_bridge/dac_bridge,
  s_xfer, gain/summer/mult, oneshot, pwm, sampled-data filter.
- M5.3: acceptance = vendored `tests/ngspice/xspice` suite green + `examples/digital` triage; document
  the supported subset vs full XSPICE explicitly.

---

## WS6 — Product, content & documentation

- M6.1 **User manual**: a real one (mdBook or similar, versioned with the repo): getting started,
  netlist reference (per-dialect tables from WS1), analysis reference with theory notes, device
  model reference (parameters, defaults, units — generated from code to prevent drift), tutorials.
  Gate: every public feature reachable from the manual; doc-coverage CI check.
- M6.2 **Model library**: curated `models/` expansion (passives with parasitics, common discretes,
  logic families, magnetics, references/regulators as native subckts), each entry smoke-tested in
  CI; `rspice fetch-model` for vendor downloads (license-respecting, local cache).
- M6.3 **UI program** (separate detailed plan owned by the UI session): schematic editor completion
  (hierarchy navigation, undo/redo robustness, bus/label ergonomics), **LTspice `.asc`/`.asy`
  import** (adoption lever #1 — read-only import is enough), simulation dashboards for MC/corners
  (histograms, scatter, yield), cross-probing netlist↔schematic↔waveforms.
- M6.4 **Distribution**: signed installers (MSI/dmg/AppImage), `winget/brew` formulas, versioned
  releases with changelogs, crash reporting (opt-in, offline-friendly), `rspice doctor`
  environment diagnostics.

---

## WS7 — Commercial qualification

- M7.1 **Reproducibility policy**: bit-identical results across runs at fixed thread count;
  documented cross-platform tolerance; pinned-toolchain release builds; numeric changes gated by
  the full corpus + benchmark scoreboards. (The wall-clock-deadline revert precedent becomes
  written policy: no timing-dependent numerics, ever.)
- M7.2 **Release engineering**: branch/tag discipline, release-candidate corpus run (full external
  corpora incl. Xyce_Regression + compat mirror), versioned file-format compatibility guarantees
  (raw/HDF5/`.rsch`), deprecation policy.
- M7.3 **Encrypted model support**: RSpice-native encrypted-library format (authenticated
  encryption, per-vendor keys) + tooling for vendors; document that third-party encrypted formats
  (HSPICE `.enc`, PSpice `.lib` encryption) are legally/technically out of reach without
  partnerships. Required before most silicon vendors will ship RSpice models.
- M7.4 **Robustness fuzzing**: parser fuzzing (cargo-fuzz) on all dialects + raw/HDF5 readers;
  malformed-input never panics (commercial-grade input handling; also the security story for
  reading untrusted netlists).
- M7.5 Stretch (post-program): aging/reliability (HCI/BTI) analysis, RC reduction for post-layout,
  AMS co-simulation — the remaining Spectre/HSPICE exclusives; explicitly out of current scope.

---

## Sequencing

```
Phase I  (foundations + adoption core):  WS0 → start WS1.1/M2.1/M0.5 in parallel lanes
Phase II (depth):                        WS1.2–1.3, WS2.2–2.3, WS3 (gated on M0.5 data), WS4.1–4.2
Phase III (breadth + polish):            WS4.3, WS5, WS6, WS7
```

Lane discipline for concurrent sessions (existing convention): parser/ingestion (WS1), veriloga +
device (WS2), engine/solver (WS3), analysis/rf + harness (WS0/WS4), ui (WS6) — disjoint crates or
module subtrees per session.

## World-class scorecard (definition of done)

| Metric | Target |
|---|---|
| ngspice regression suite | 113/113 under documented adjudication policy |
| Foundry PDKs | sky130 + GF180 + IHP SG13G2: corners, MC, RF decks green; zero ignored-parameter warnings |
| CMC models | PSP, HICUM/L2, BSIM-CMG pass shipped frontier decks and public/oracle corpora through the VA-to-Rust pipeline; private official QA decks are an optional gate if access becomes available (generated eval JIT <=2x native eval) |
| Vendor-model ingestion | ≥90% of top-100 compat corpus parses & matches reference waveforms |
| Performance | ≥1.0× ngspice-KLU single-thread geomean on macro bench; ≥6×/8-core MC; 100k-node tran ≤4× ngspice |
| RF | ≥40-deck RF suite green incl. analytic Tier-0 + Xyce cross-oracle |
| Mixed-signal | vendored xspice suite green |
| Product | user manual covers 100% of public surface; signed installers; .asc import |
| Robustness | fuzzing: zero panics on malformed input; bit-reproducible runs |
