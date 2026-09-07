# CI coverage and qualification

`CI`, `python`, and `Security` validate changes to `main` and pull requests.
Rust dependency resolution is locked; the toolchain and action revisions are
pinned. Actionlint checks workflow syntax. Configuration tests guard coverage.

| Lane | Responsibility |
| --- | --- |
| Quality | Every handwritten member's formatting; workspace Clippy over all targets; tooling, browser worker, packaging and runtime harness tests; model provenance and licensing |
| Core, Linux | All core unit, integration and documentation tests; routine digital and mixed-signal conformance and execution corpora |
| UI, Linux | All default UI unit, integration and documentation tests; generated-catalog UI contracts |
| Remaining workspace, Linux | Every other member's default tests, including output, design, automation, cloud, engine adapter, publishers and viewers; new members are selected automatically |
| Generated models | Portable feature shards, full catalog validation, XSPICE registry, resource and numerical checks, representative performance qualification |
| Desktop | Windows/macOS UI and CLI tests; OS-specific output and managed-runtime tests; native JIT execution on six OS/architecture combinations; macOS hardened-runtime checks |
| Browser | wasm32 compilation, optimized UI/worker size budgets, clocks in a window and worker, Chromium solver/JIT execution, real workbench review and recovery |
| Mobile | ARM64 Android and iOS portable solver/catalog compilation |
| Feature shards | Clippy for configurations hidden by workspace feature unification; executable tests for host configurations |
| Python | Supported CPython versions including free threading; Rust binding invariants, stub validation, wheels on six native platforms, offline source-distribution installation |
| Security | Committed dependency graph, advisory/license/source policies, expiring exceptions, PR dependency review and SBOMs |

The remaining-workspace selector excludes crates with dedicated lanes. Generated
model packages and catalog utilities are qualified through the simulator and
catalog gates instead of empty generated-crate test binaries. The WASM surface
has its own host tests and lint lane.
Browser download budgets measure the production `_bg.wasm` modules emitted by
wasm-bindgen, before rebuilding the UI with qualification instrumentation.
The Cargo linker output includes binding metadata removed before delivery.

Nightly adds release-mode tests, ngspice/Xyce/GF180 corpora, full execution depth,
panic checks, generated-model freshness, golden fingerprints, complex-step
Jacobians, native device qualification, performance and shipping desktop builds.
It installs Icarus Verilog and Verilator and requires both for independent
digital conformance; a missing reference simulator fails the qualification.
A failed test process fails the job even if it printed a successful subtotal.
Performance thresholds remain enforced alongside numerical checks.

Coverage is a weekly or manually dispatched report, split into core, UI and
remaining-library artifacts. All of those tests also run in per-change CI.
There is no percentage threshold: a second full instrumented build on every
push added substantial cost without a distinct acceptance criterion. Generated
packages and the conformance harness are not coverage-report targets.

Drawing-sheet qualification remains manual for checksummed evidence across
seven targets. Tagged native releases and manual component releases retain
their package, provenance, signature and immutability checks. Publishing
workflows must not be dispatched merely to test CI.
Native publication also requires the reusable CI and nightly qualification
workflows to pass for the release commit; building an archive is insufficient.

Browser automation currently uses Chromium. Mobile rows prove portable solver
compilation, not a native tablet application. WebKit/Safari, Firefox, physical
tablet interaction, assistive technology, installer signing/notarization and
sustained performance on controlled hardware still need product qualification.
A green workflow alone does not establish production readiness.
