RSpice test corpora
===================

This directory contains vendored upstream simulator test corpora. Each corpus
keeps its own upstream layout, provenance, and license terms.

Corpora divide into two kinds, and the distinction decides what a green run
means.

**Reference corpora** ship another simulator's published output, so RSpice can
be asked whether its numbers are right:

- `ngspice/` — the ngspice-46 regression corpus, with checked-in `.out`
  tables. Run by `tools/rspice-conformance/tests/ngspice_regression.rs`.
- `xyce/` — the Xyce Regression Suite runtime corpus, with `.prn` references.
  Run by `tools/rspice-conformance/tests/xyce_regression.rs`.
- `gf180mcu/` — GlobalFoundries' open 180nm PDK with per-case ngspice-46
  reference curves. The material is what is new here, not the oracle: BSIM4
  through the vendor's own subcircuit wrappers, process corner libraries, and
  characterisation sweeps from -40C to 175C. Apache-2.0, and the only vendored
  corpus RSpice may redistribute without qualification. Run by
  `tools/rspice-conformance/tests/gf180mcu_devices.rs`.

**Self-oracled corpora** arrive from upstream with no reference output, so
RSpice captured its own with ngspice 47 and checked the tables in as
`.oracle.out` sidecars — 138 across `paranoia/`, and 23 of the 24 standalone
`iscas85/` decks. Where a sidecar exists the runner compares numerically, the
same claim a reference corpus supports; the difference is that RSpice recorded
the oracle rather than receiving it. A deck without one is execution-only: the
run answers whether it loads, builds, and either completes or refuses cleanly,
which means RSpice survived the deck, not that it was right about it. Each
corpus's vendoring note records the capture binary and its digest. Both are run
by `tools/rspice-conformance/tests/execution_corpora.rs`:

- `iscas85/` — the ISCAS85 benchmarks as transistor-level SPICE. Bought for
  scale: up to ~89k netlist lines and thousands of devices, an order of
  magnitude past anything else here. **Licensing unresolved — see its
  vendoring note before shipping the `tests/` tree.**
- `paranoia/` — the ngspice example decks. Bought for dialect breadth: CIDER,
  `.measure`, transient noise, XSPICE, Monte Carlo, memristors. Modified BSD.

**Live-oracle corpora** carry no reference output and never will, because the
oracle is a program rather than a file:

- `verilog/` — digital Verilog run through Icarus Verilog and Verilator, two
  simulators that share no code with RSpice or with each other. RSpice-authored
  rather than vendored, and chosen for language coverage rather than scale: one
  case per construct the digital front end implements. Run by
  `tools/rspice-conformance/tests/verilog_oracles.rs`.

  The difference that matters is what a green run means when the oracles are
  absent, which on CI they always are — neither binary is installed there.
  Without them the suite still checks corpus integrity and testbench
  generation, and checks RSpice's traces against expectations derived by hand
  from IEEE 1364-2005 rather than produced by RSpice; the cross-simulator
  comparisons report what they looked for and return, so a green run does
  **not** mean RSpice agreed with Icarus or Verilator.
  `RSPICE_VERILOG_ORACLES_REQUIRED=1` turns that absence into a failure.
  RSpice's arm runs behind the conformance crate's `verilog-digital` feature,
  which is on by default; a design the digital front end cannot compile becomes
  a named refusal carrying the engine's diagnostic, never a pass.

**The mixed-signal reference corpus** combines all three ideas:

- `mixed-corpus/` — twelve designs (`c01`–`c12`) that exercise SPICE, XSPICE
  and Verilog-AMS together, plus the `probes/` boundary cases. Each design carries
  its decks, oracle decks, checked-in ngspice/Xyce/co-simulation references,
  a `tolerance.toml` fixed before the first comparison, and `outcomes.tsv`:
  the recorded pass/refused/fail/wrong/unverified outcome of every design ×
  variant × analysis × route cell. Analytic expectations live in
  `tools/rspice-conformance/src/suites/mixed/analytic/`. Run by
  `tools/rspice-conformance/tests/mixed_corpus.rs` (nightly, once per route);
  the integrity checks run with the crate's unit tests. Derived from the
  ngspice examples (Modified BSD, `LICENSE-ngspice`) and RSpice-authored;
  `RSPICE-VENDORING.md` records every edit.

Do not mix validation manifests, generated outputs, or harness sidecars between
corpora. Add a corpus-specific README or vendoring note when importing another
upstream suite, and record excluded material there — `paranoia/` in particular
drops third-party proprietary device models that upstream redistributes, and a
re-vendoring that skips that step reintroduces them.
