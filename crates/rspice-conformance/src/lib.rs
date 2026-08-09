//! # RSpice Conformance
//!
//! The regression suites that validate RSpice against reference simulators.
//! Each suite discovers a vendored deck corpus, runs every deck through
//! [`rspice_core`], and compares the result against a reference contract.
//!
//! ## Why this is a separate crate
//!
//! A conformance suite exists to answer one question: does the simulator a
//! user gets produce the right numbers? A suite that can reach into engine
//! internals cannot answer it — assertions on private state survive real
//! numerical regressions, because they never travel the path a user's deck
//! takes. Living outside `rspice-core` makes that a compiler-enforced
//! property rather than a convention: these suites can only see the public
//! API, so every assertion is made through the same surface a frontend uses.
//!
//! The dependency direction is the whole point, and it is one-way. This crate
//! depends on `rspice-core`; nothing depends on this crate. No shipping
//! crate — the CLI, the GUI, the Python or WASM bindings — may take a
//! dependency on it, which `tools/ci/test_ci_configuration.py` asserts.
//!
//! ## Corpora
//!
//! Deck corpora are vendored at the workspace root, not inside this crate:
//! `tests/ngspice/`, `tests/xyce/`, `tests/gf180mcu/`, `tests/iscas85/`, and
//! `tests/paranoia/`.
//! They are shared data with their own licensing and provenance files, and
//! they outlive any one runner. Each carries an `RSPICE-VENDORING.md`
//! recording what was trimmed and why; `tests/paranoia/`'s in particular
//! records third-party proprietary device models that upstream redistributes
//! and RSpice must not.
//!
//! ## Suites
//!
//! - [`suites::ngspice`] — the ngspice conformance corpus
//! - [`suites::xyce`] — the Xyce conformance corpus
//! - [`suites::gf180mcu`] — a released foundry PDK (GlobalFoundries 180nm)
//!   evaluated against per-case ngspice references
//! - [`suites::execution`] — the corpora that carry no reference data:
//!   ISCAS85 for scale, the ngspice examples for dialect breadth
//! - `suites::veriloga` — the generated Verilog-A built-ins, behind
//!   `veriloga-builtins-base`
//!
//! Three different instruments, and it is worth being precise about which
//! answers what. The first two compare RSpice against *another simulator's*
//! published output. The Verilog-A suite compares each generated device
//! against RSpice's own captured stamps plus a finite-difference oracle that
//! shares no code with the chain rule under test — snapshot and independent
//! mathematics rather than external conformance, which is the right tool when
//! the thing under test is a code generator and there is no second simulator
//! computing the same Jacobian to ask.
//!
//! [`suites::execution`] is the weakest of the four and says so: its corpora
//! ship no reference data, so it can only establish that a deck loaded, built,
//! and either ran or refused cleanly. It earns its place because the decks
//! themselves are unlike the others — a reference suite's decks are, by
//! construction, ones their authors could already simulate, while these are
//! real circuits reaching for whatever dialect corner they happened to need.
//! Every failure there is a named gap in ingestion, device coverage, or solver
//! robustness.
//!
//! `rspice-core` now contains no validation harness at all, which
//! `tools/ci/test_ci_configuration.py` also asserts.
//!
//! ## Dependency direction
//!
//! Nothing a user installs may depend on this crate. `rspice-bench` does,
//! deliberately: it is a benchmark rig rather than a product, and its
//! generated-stamp timing harness consumes the hand-written Verilog-A suite.
//! Golden capture, replay, and derivative audit are exposed by the
//! `rspice-veriloga-golden` conformance binary and remain wholly owned here.
//!
//! ## Feature forwarding
//!
//! Both suites branch on `rspice-core` features to decide whether a deck
//! should run or be recorded as unsupported. Every such feature is
//! re-declared here and forwarded, because an undeclared `feature = "x"`
//! evaluates to `false` in this crate — the suite would take the
//! "unsupported" branch and pass vacuously even when the models were
//! compiled in. A conformance suite that quietly skips cases is the one
//! outcome it must never have, so CI asserts the forwarding is complete.
//!
//! ## Process isolation
//!
//! A deck that panics, hangs, or overflows its stack must not take the suite
//! down with it, so the integration tests spawn one child process per deck.
//! The child is the `rspice-ngspice-case-runner` binary; the parent decodes a
//! result file it writes. A missing or unparsable result file is how the
//! parent detects a crash.

pub mod suites;
