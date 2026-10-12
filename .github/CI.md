# CI coverage and qualification

`CI`, `python`, and `Security` validate changes to `main` and pull requests.
Rust dependency resolution is locked; the toolchain and action revisions are
pinned. Actionlint checks workflow syntax. Configuration tests guard coverage.

| Lane | Responsibility |
| --- | --- |
| Quality | Every handwritten member's formatting; workspace Clippy over all targets; tooling, browser worker, packaging and runtime harness tests; model provenance and licensing |
| Core, Linux | All default-feature core unit, integration and documentation tests; routine digital and mixed-signal conformance and execution corpora |
| Verilog-A & mixed signal, Linux | Every `veriloga`-gated core integration suite through the bytecode interpreter; the desktop lane runs the same suites through the x64 JIT |
| UI, Linux | All default UI unit, integration and documentation tests; generated-catalog UI contracts |
| Remaining workspace, Linux | Every other member's default tests, including output, design, automation, cloud, engine adapter, publishers and viewers; new members are selected automatically |
| Generated models | Portable feature shards, full catalog validation, XSPICE registry, resource and numerical checks, representative performance qualification |
| Desktop | Windows/macOS UI and CLI tests; OS-specific output and managed-runtime tests; native JIT execution on six OS/architecture combinations; macOS hardened-runtime checks |
| Browser | wasm32 compilation, production UI/worker size reports, Firefox Rust binding/cloud/clock tests, Chromium solver/JIT execution and workbench review/recovery |
| Mobile | ARM64 Android and iOS portable solver/catalog compilation |
| Feature shards | Clippy and host tests for configurations hidden by workspace feature unification, including emitted WebAssembly execution in wasmi |
| Python | Supported CPython versions including free threading; Rust binding invariants, stub validation, wheels on six native platforms, offline source-distribution installation |
| Security | Committed dependency graph, advisory/license/source policies, expiring exceptions, PR dependency review and SBOMs |

The remaining-workspace selector excludes crates with dedicated lanes. Generated
model packages and catalog utilities are qualified through the simulator and
catalog gates instead of empty generated-crate test binaries. The WASM surface
has its own host tests and lint lane.
Generated bundle freshness runs through `generated_output_audit` in the
workspace tests; a second generator build solely to repeat that audit is omitted.
Broad test commands finish the remaining test binaries after a failure so one
broken suite does not hide independent findings.
Conformance's default features already include the Verilog-A oracle and admitted
generated models; its library tests run once, without a duplicate base-feature pass.
The two optional BSIM4 Verilog-A oracle suites are the only `veriloga`-gated core
targets neither Verilog-A lane names: their model source is externally supplied
rather than vendored, so every pin in them returns without measuring anything.
Browser size reports measure the production `_bg.wasm` modules emitted by
wasm-bindgen, before rebuilding the UI with qualification instrumentation.
The Cargo linker output includes binding metadata removed before delivery.
Raw and gzip sizes are reported without arbitrary blocking caps. Hosting upload
limits belong in the deployment pipeline for the actual packaged assets; this
repository does not deploy the browser application.

The project infrastructure budget is **$0**. Use standard GitHub-hosted runners
while the repository is public. Do not introduce paid runners, paid storage, or
usage-based services with billable overages as CI or deployment dependencies.
A service's free allowance alone does not guarantee a zero bill.

Nightly adds release-mode tests, ngspice/Xyce/GF180 corpora, full execution depth,
panic checks, generated-model freshness, golden fingerprints, complex-step
Jacobians, native device qualification, performance and shipping desktop builds.
It installs Icarus Verilog and Verilator and requires both for independent
digital conformance; a missing reference simulator fails the qualification.
A failed test process fails the job even if it printed a successful subtotal.
Generated-model timings, source/file counts and per-instance state sizes are
reported without universal caps. Comparing different compact-model equations to
one hand-written BSIM4 model is useful context, but does not define a common
performance requirement. The benchmark still rejects failed initialization,
non-finite stamps and models that produce no contribution. Zero pooled scratch
workspace remains an architectural check against restoring persistent per-thread
work buffers. Manifest authentication and the complete model census also remain
mandatory.
Native JIT speedup checks compare the same model and operation against bytecode
on the same runner, with output agreement checked before timing. These relative
checks remain; absolute setup/latency, sample dispersion and code-image sizes
are reported without host-independent caps. Explicit budget options are available
for controlled-hardware qualification. The published viewer runtime likewise
reports its compressed size without an arbitrary download cap.
KLU's hosted-runner gate compares refactor and solve medians with full
factorization on the same runner (at most 50% and 25%, respectively), while
retaining zero-fallback, backward-error and fill checks. The benchmark's absolute
nanosecond options remain available for qualification on controlled hardware.
Absolute latency is still reported in the CI artifact.
Numerical tolerances, complete-corpus counts, platform upload limits and bounded
job/deck timeouts have correctness, coverage or resource-safety purposes and are
not removed by the reporting policy.

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

Workbench automation currently uses Chromium; Rust boundary tests use Firefox.
Mobile rows prove portable solver compilation, not a native tablet application.
WebKit/Safari, full Firefox workbench interaction, physical
tablet interaction, assistive technology, installer signing/notarization and
sustained performance on controlled hardware still need product qualification.
A green workflow alone does not establish production readiness.
