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
//! `tests/ngspice/` and `tests/xyce/`. They are shared data with their own
//! licensing and provenance files, and they outlive any one runner.
//!
//! ## Suites
//!
//! - [`suites::ngspice`] — the ngspice conformance corpus
//! - [`suites::xyce`] — the Xyce conformance corpus
//! - `suites::veriloga` — the generated Verilog-A built-ins, behind
//!   `veriloga-builtins-base`
//!
//! The first two compare RSpice against *another simulator's* published
//! output. The Verilog-A suite is a different instrument and is kept
//! distinct on purpose: it compares each generated device against RSpice's
//! own captured stamps, plus a finite-difference oracle that shares no code
//! with the chain rule under test. Snapshot and independent mathematics
//! rather than external conformance — which is the right tool when the thing
//! under test is a code generator, since there is no second simulator
//! computing the same Jacobian to ask.
//!
//! `rspice-core` now contains no validation harness at all, which
//! `tools/ci/test_ci_configuration.py` also asserts.
//!
//! ## Dependency direction
//!
//! Nothing a user installs may depend on this crate. `rspice-bench` does,
//! deliberately: it is a benchmark rig rather than a product, and the
//! Verilog-A stamp bench and golden-fingerprint capture belong with the suite
//! rather than inside the library they measure.
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
