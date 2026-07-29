RSpice test corpora
===================

This directory contains vendored upstream simulator test corpora. Each corpus
keeps its own upstream layout, provenance, and license terms.

Corpora divide into two kinds, and the distinction decides what a green run
means.

**Reference corpora** ship another simulator's published output, so RSpice can
be asked whether its numbers are right:

- `ngspice/` — the ngspice-46 regression corpus, with checked-in `.out`
  tables. Run by `crates/rspice-conformance/tests/ngspice_regression.rs`.
- `xyce/` — the Xyce Regression Suite runtime corpus, with `.prn` references.
  Run by `crates/rspice-conformance/tests/xyce_regression.rs`.
- `gf180mcu/` — GlobalFoundries' open 180nm PDK with per-case ngspice-46
  reference curves. The material is what is new here, not the oracle: BSIM4
  through the vendor's own subcircuit wrappers, process corner libraries, and
  characterisation sweeps from -40C to 175C. Apache-2.0, and the only vendored
  corpus RSpice may redistribute without qualification. Run by
  `crates/rspice-conformance/tests/gf180mcu_devices.rs`.

**Oracle-free corpora** ship no reference data at all, so the only question
they can answer is whether every deck loads, builds, and either completes or
refuses cleanly. That is a weaker claim, and it is stated weakly on purpose —
a green run means RSpice survived the deck, not that it was right about it.
Both are run by `crates/rspice-conformance/tests/execution_corpora.rs`:

- `iscas85/` — the ISCAS85 benchmarks as transistor-level SPICE. Bought for
  scale: up to ~89k netlist lines and thousands of devices, an order of
  magnitude past anything else here. **Licensing unresolved — see its
  vendoring note before shipping the `tests/` tree.**
- `paranoia/` — the ngspice example decks. Bought for dialect breadth: CIDER,
  `.measure`, transient noise, XSPICE, Monte Carlo, memristors. Modified BSD.

Do not mix validation manifests, generated outputs, or harness sidecars between
corpora. Add a corpus-specific README or vendoring note when importing another
upstream suite, and record excluded material there — `paranoia/` in particular
drops third-party proprietary device models that upstream redistributes, and a
re-vendoring that skips that step reintroduces them.
