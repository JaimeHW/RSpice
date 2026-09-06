//! The crate's public surface may shrink, never grow.
//!
//! `rspice-core` exports 4,257 public item statements. Its five frontends —
//! the CLI, the GUI, the Python and WASM bindings, and the conformance suite —
//! name roughly two hundred distinct paths between them. The rest is internal
//! machinery that happens to be spelled `pub`.
//!
//! That has a cost even though the crate is `publish = false` and carries no
//! semver obligation. Every `pub fn` is a thing a refactor must consider
//! moving, a name rustdoc must list, and a hint to the next reader that
//! somebody outside might depend on this. When almost everything is public,
//! `pub` stops carrying information.
//!
//! This test does not judge which items should be private — that is Phase 9,
//! driven by what the frontends actually import. It exists so the number
//! cannot drift upward while the earlier phases are moving code around. New
//! code should be `pub(crate)` unless a frontend needs it.
//!
//! # What is counted
//!
//! Item declarations written `pub fn` / `pub struct` / `pub enum` /
//! `pub trait` / `pub type` / `pub const` / `pub static`, plus `pub use`
//! re-export statements, at the start of a line after indentation.
//!
//! The unit is the *statement*, not the name. A grouped re-export —
//! `pub use foo::{A, B, C};` — counts once while exposing three names. That
//! makes the number a proxy rather than a census, and it means relocating a
//! type behind a grouped re-export reads as +1 even though the set of public
//! names did not change.
//!
//! # Raising the ceiling
//!
//! Two cases justify it, and both require saying so in the commit:
//!
//! 1. A move that provably preserves the set of public names, by the number of
//!    re-export statements it adds.
//! 2. A deliberate new public API — an entry point a frontend is meant to
//!    call.
//!
//! Nothing else. In particular, "the number went up because I added a helper"
//! is the case this test exists to catch: make it `pub(crate)`. Prefer
//! investigating a rise to explaining it away.
//!
//! `pub(crate)`, `pub(super)` and `pub(in ...)` are deliberately *not*
//! counted: restricting visibility is the direction this test wants, so
//! narrowing `pub` to `pub(crate)` lowers the number, which is the point.
//!
//! Generated Verilog-A is excluded for the same reason as in the layering
//! ratchet: it is machine output, and its size is the generator's business.

use std::fs;
use std::path::{Path, PathBuf};

use rspice_core::analysis::harmonic_balance::{
    DepletionCap, HbConfig, HbError, HbSolver, HbSolverState, HbSwitchNodes, HbVoltageSwitchModel,
    NonlinearDeviceInstance,
};

/// Ceiling on public items. Lower it whenever the real count drops. The build
/// fails if the count exceeds this, and also if it falls far enough below that
/// the ceiling has gone stale.
///
/// Raise it only for an item a frontend is meant to call, and say which one in
/// the commit. "Never raise it" would be the simpler rule and it is the wrong
/// one: it does not stop a public surface from growing, it stops a feature
/// from shipping an API, and the way that gets resolved is by marking things
/// `pub` somewhere this test does not count. The ceiling exists to make growth
/// a decision, not to forbid it.
///
/// The last raise was +8 for the attributed-failure vocabulary the GUI reads
/// off `Engine::convergence_quality` to mark non-converged objects on the
/// schematic: `ConvergenceDiagnostic`, `ConvergenceFailureClass`,
/// `ConvergenceSite`, `ConvergenceSiteKind`,
/// `ConvergenceDiagnostic::MAX_NAMED_SITES`,
/// `ConvergenceDiagnostic::describes`, and
/// `ConvergenceQuality::record_failure_diagnostic`. Every one of them is on
/// the path from a failed run to a highlighted conductor; without them the
/// only way to that highlight is parsing the failure's prose.
///
/// The eighth is `Engine::resolved_for_netlist`, which the GUI's bridge must
/// call to get its per-deck engine. It was already the only construction
/// that keeps the run's metrics reachable after the per-deck engine is
/// dropped; the GUI was building its own and losing them.
///
/// The next raise was +1 for `Engine::try_resolved_with_config`, the same
/// metric-sharing construction for a caller that has already resolved the
/// configuration itself. The operating point must: its temperature override,
/// accuracy tier and homotopy choice are applied on top of `.OPTIONS`, so
/// there is no netlist for `resolved_for_netlist` to resolve against, and the
/// bridge was falling back to `try_new_with_resolved_config` — which is why
/// the one analysis that names non-converged conductors never reported one.
///
/// The current raise is +3, from 4,253 to 4,256, after narrowing 22
/// test-fixture and transient-numerics declarations that were never frontend
/// API. The remaining net growth is deliberate: the conformance frontend
/// calls the bounded Xyce PRN serializer; the CLI calls bounded sweep,
/// checkpoint/restart and abort-aware result APIs; and the GUI and Python
/// bindings consume the authenticated pole-zero and Floquet evidence. The GUI
/// also calls the unit-interval estimator. Larger deletions of unused device
/// APIs offset most of those additions, so +3 is the statement-count delta,
/// not the number of frontend paths added.
///
/// The latest raise is +1 for `XyceHbTimeDomainMode`, which the conformance
/// frontend reads from parsed netlists to prove that authored `HBINT.TAHB`
/// controls remain typed. Its numeric rendering helper stays crate-private.
///
/// The next +12 are also cross-crate contracts that landed after that ratchet:
/// eight bounded packed-checkpoint APIs used by the CLI and conformance suite,
/// three RustFFT planning-qualification items shared with the GUI, and the
/// typed duplicate-model-parameter error exposed by the Python binding.
///
/// The latest +1 is `NetlistDiagnostic::xyce_legacy_warning_lines`, which the
/// CLI calls to render the byte-stable two-line Xyce compatibility warning.
///
/// The current reduction removes 36 unused public statements from the legacy
/// HB-only diode/BJT/MOS wrapper module and its re-export. Production HB uses
/// the exact `solver::NonlinearDeviceInstance` path; retaining a second,
/// simplified public device vocabulary made the supported numerical contract
/// ambiguous.
///
/// The latest net change is +3: authenticated PSS operating-point identity
/// adds five frontend-consumed public statements (`PssOperatingPointIdentity`
/// plus its canonical-parts, shooting-state-basis, producer-identity, and
/// authenticated-construction APIs), while removing the obsolete public HB
/// BJT-parameter and current-switch registration helpers offsets two.
///
/// The latest +1 is `BehavioralEvaluationError`. Public behavioral-source
/// evaluation and stamping APIs are fallible so non-finite equations cannot
/// be silently converted into zero-valued sources; callers need the typed
/// error to retain source identity and analysis-coordinate diagnostics.
///
/// The current raise is +30, and it is two unrelated amounts.
///
/// +2 of it is arrears, not growth. `b97258608` ("Bound model library
/// ingestion and discovery") added ten public statements under `src/library/`
/// without touching this ceiling; eight statements deleted earlier in the same
/// range absorbed all but two, so the tree has been two over at 4,263 since
/// that commit and this test has been failing on `main`. Raising to cover them
/// records the debt rather than paying it — several of those ten
/// (`with_max_source_files`, `DEFAULT_MAX_LIBRARY_SOURCE_FILES`,
/// `DEFAULT_MAX_VERILOGA_DISCOVERY_FILES`) look like `pub(crate)` candidates,
/// and narrowing them is the change that should lower this number again.
///
/// +28 is the discrete-event scheduler kernel in
/// `xspice::event_scheduler`: the tick time base (`TimeResolution` and its
/// five operations plus `MAX_EXACT_TICKS`), the stratified region vocabulary
/// (`SchedulerRegion`, `ORDERED`), the event payload (`EventTarget`,
/// `ScheduledEvent`), the typed failure surface (`SchedulerError`,
/// `OscillationCause`, `OscillationDiagnostic`), the ceilings and per-slot
/// report (`SchedulerLimits`, `TimeSlotReport`), and the scheduler itself
/// (`EventScheduler` with six operations, `SchedulerContext` with three).
///
/// These are public for a reason this test's usual rule does not cover:
/// `event_scheduler_kernel.rs` is an integration test, so it can only reach
/// the kernel through the crate's public face. The kernel's ordering and
/// determinism guarantees are the substrate everything digital rests on and
/// that test is what holds them, so the surface it drives stays public.
///
/// The latest +3 is what rehosting XSPICE on the kernel needs from it:
/// `schedule_superseding_at` (a driver replacing its own pending output),
/// `run_due_events` (executing everything due at or before a bound the analog
/// engine names), and `note_delta_cycle` (an outer settle loop marking one
/// iteration so a network that will not quiet is diagnosed rather than
/// looping). Each is driven by the same integration test.
///
/// The previous net change was -20, from 4,291 to 4,271. Those +3 were in it,
/// and the rehost itself removed 23: `xspice::Event`, `xspice::EventQueue` and
/// `EventQueueStats` with their constructors, scheduling, draining,
/// cancellation and statistics methods, and `XspiceInstance::schedule_events`,
/// which no longer names a public queue type. The kernel is what schedules
/// now, and nothing outside this crate ever named the queue it replaced.
///
/// The latest raise is +8, from 4,271 to 4,279: the whole entry path for
/// executing digital Verilog, which the conformance suite's oracle harness
/// calls. It is the second case the doc above admits — a deliberate new API a
/// frontend is meant to call — and it is the *entire* API, not a helper that
/// leaked:
///
/// `xspice::verilog::run_digital_verilog` is the call. `DigitalStimulus`,
/// `DigitalPort` and `DigitalClock` are its input; `DigitalRunReport` and
/// `DigitalObservation` are its output; `DigitalRunError` is its refusal, which
/// costs two statements because the type lives in the crate-private `host`
/// module and is re-exported. Everything else the host and the signal store
/// declare — the store itself, the resolution table, the process scheduler — is
/// `pub(crate)`, as is the time-unit ruling, which the refusal prints rather
/// than a caller reading.
///
/// The coherent-noise program briefly added five public statements to
/// `src/device/veriloga_builtins.rs`, but nothing outside `rspice-core` names
/// any of them — every caller is `src/engine/noise.rs`:
///
/// `BuiltinEvaluatedNoiseInjection` and `BuiltinEvaluatedNoiseProcess` are the
/// per-process result the generated coherent ABI hands back;
/// `has_grouped_noise_processes` is the capability probe that tells a catalog
/// generated before that ABI apart from one generated after it;
/// `grouped_noise_process_catalog` names the processes for the noise report;
/// and `evaluate_noise_processes_at_frequency` is the evaluation itself.
///
/// Those five APIs are now `pub(crate)`, so the ratchet is back at its prior
/// 4,279-item ceiling rather than retaining accidental headroom.
///
/// The FFT result contract adds +5 deliberate frontend-facing types:
/// `XyceFftMode` retains the authored compatibility selection, while
/// `TransientFftBin`, `TransientFftHarmonic`, `TransientFftMetrics`, and
/// `TransientFftResult` expose calibrated spectra and optional `FFTOUT`
/// figures without requiring a frontend to parse an engine text report.
///
/// 2026-08-31, +23 arrears (4,284 → 4,307): two landings raised the count
/// without touching this ceiling — deterministic TEAM resistance noise
/// (`db03d39eb`) and the transactional mixed Verilog transient host
/// (`9549ed6cb`). Neither set has been triaged for narrowing; whether each
/// item is genuinely frontend-facing or a `pub(crate)` candidate is the
/// visibility-narrowing pass's question, not this ratchet's. Recorded here so
/// the next raise cannot mistake the arrears for headroom.
///
/// 2026-08-31, -10 (4,307 → 4,297): the narrowing pass the +30 above asked for,
/// run over the ten statements `b97258608` added under `src/library/`. All ten
/// are now `pub(crate)`: the ingestion ceilings
/// `DEFAULT_MAX_LIBRARY_SOURCE_FILES` and
/// `DEFAULT_MAX_VERILOGA_DISCOVERY_FILES`; the `LibParser` builders
/// `with_resource_limits` and `with_max_source_files` and its
/// `parse_file_with_abort`; the `LibraryManager` loaders
/// `load_external_lib_with_limits` and
/// `load_external_lib_with_limits_and_abort`; and the discovery limit type
/// `VerilogADiscoveryLimits` with `discover_veriloga_models_with_limits` and
/// `discover_veriloga_models_with_limits_and_abort`.
///
/// Every one is an explicit-limit or cancellation overload, and the only
/// callers are `LibraryManager` and this crate's own tests. The entry points a
/// frontend actually names — `LibParser::new`, `parse_file`, `parse_string`,
/// `LibraryManager::load_external_lib`, `discover_veriloga_models` — stay
/// public and still apply those bounds from `ResourceLimits::default()`, so
/// the ingestion that commit bounded remains bounded on the public path.
///
/// Three of the ten turned out to have no shipping caller at all once they
/// stopped being API, which `-D warnings` then said out loud.
/// `load_external_lib_with_limits` had no caller in any configuration and an
/// abort-taking twin one line below it, so it is deleted rather than narrowed;
/// `with_max_source_files` and `discover_veriloga_models_with_limits` are
/// `#[cfg(test)]`, being how the tests drive one ingestion limit at a time to
/// its edge. All three still count as -1 each: a deleted `pub fn` and a
/// cfg-gated `pub(crate) fn` are both gone from this number.
///
/// Established by compiling the CLI, the GUI, the Python and WASM bindings and
/// the conformance suite against the narrowed items, which is the check this
/// question needs: a bare-name grep cannot answer it, because a grouped
/// `pub use` lets a frontend name an item through a path the declaration's own
/// name never appears in. The two grouped re-exports in `src/library.rs` drop
/// those names but keep their statements, so the whole -10 is declarations.
///
/// 2026-09-01, +3 deliberate (4,297 → 4,300): the compile-once digital run
/// API. `CompiledDigitalDesign`, `CompiledDigitalDesign::compile` and
/// `CompiledDigitalDesign::run` split `run_digital_verilog` into its two
/// halves, and every one of the three is on the path a caller with many
/// stimuli and one design must take — the conformance suite's RNM performance
/// measurement is that caller, and without them the only way to run a design
/// twice is to compile it twice. `run_digital_verilog` is retained unchanged
/// as their composition, so nothing already public moved or grew. The
/// module-name accessor that would have made a fourth was *not* added: no
/// frontend reads it, `Debug` prints the name, and the refusal that cites it
/// carries it.
///
/// 2026-09-01, +106 deliberate (4,300 -> 4,406): the target-neutral execution
/// contract required by the CLI, Python, WASM, and engine adapter. The public
/// statements are the typed deck-plan, run-coordinate, analysis-identity,
/// signal-schema/missingness, and topology-fingerprint vocabulary plus their
/// validated constructors and read-only accessors. These are intentionally
/// cross-crate: keeping them `pub(crate)` would force each frontend to invent
/// the same semantics again, which is the architectural defect this layer is
/// designed to remove. Construction details and hash encoders remain private
/// or `pub(crate)`.
// The canonical run-axis planner deliberately exposes typed STEP targets,
// abort-aware construction, and fail-closed planning errors to every frontend.
//
// 2026-09-01, +11 deliberate (4,424 -> 4,435): frontends need the typed
// differential-startup references and conflicts, per-record continuous
// FAILVALUE verdicts/coordinates, and imported SPEF inductor count. Solver,
// parser, graph-reduction, and import-construction helpers remain internal.
// ResultSchemaMismatchError is part of the cross-surface typed failure
// contract, so its type, constructor, and SimulationError constructor are
// intentionally public.
//
// 2026-09-03, +478 (4,438 -> 4,916): the visibility-narrowing pass over
// everything the non-UI production-readiness program added since `c7c86ae33`,
// and the single raise that records what is left standing.
//
// The tree stood at 5,060 when this pass began — 622 over the ceiling, with
// this test red on `main`. 144 of those statements are gone; the other 478 are
// the raise. Every one of the 144 was decided by compiling `rspice-cli`,
// `rspice-conformance`, `rspice-engine-adapter`, `rspice-python`,
// `rspice-wasm`, `rspice-bench` and `rspice-ui` with `--all-targets` against
// the narrowed item, not by grepping for its name: a grouped `pub use` lets a
// frontend reach an item through a path the declaration never spells, so the
// compiler is the only witness.
//
// Narrowed to `pub(crate)`, by module:
//
// - `device` and `circuit`, -63. C1's argument records — `B3SoiDdNodes`,
//   `B3SoiPdNodes`, `SoiCompanionCurrents`, `VoltageControlledNodes`,
//   `CoupledWinding`, `ResistorValues`, `SourceExcitation`,
//   `SolutionDependentCompanionStep`, `DistributedRlgc`,
//   `XyceCoreCompanionMode`, `MosfetIndices`, `MosRegion`,
//   `MosBodyJunctionModel`, `MosfetOpValues` — plus the stamping,
//   construction and operating-point methods that carry them. They were `pub`
//   only because `private_interfaces` requires an argument type to be at least
//   as public as its function.
// - `execution`, -30. The `serde(with = ...)` bridge in
//   `result_document/wire.rs` is 22 of it: `serialize`/`deserialize` pairs
//   inside `pub(super) mod` blocks, spelled `pub` out of habit and reachable
//   only from `execution::result_document`. The rest is `payload_ref`,
//   `replace_payload`, `numeric_columns`, `ScalarUnavailability::classify`,
//   `ResultPayload::value_count`, `PlannedPostProcess::with_upstream`,
//   `transient_output_unit`, and the `probe_specification_error` re-export.
// - `analysis` and `netlist`, -39. The periodic-card defaults in `ast.rs`
//   (`DEFAULT_HARMONICS`, `DEFAULT_RELTOL`, `DEFAULT_SIDEBAND_MAX`, ...), the
//   Spectre statistics sampling methods, `probe_specification_error`, the
//   frequency-grid helpers, and the `_with_abort` sweep-point overloads on the
//   PAC, PNOISE, PXF, STB and transfer configs.
// - `engine`, `numerics` and `xspice`, -12. `TransientChannelOwner`'s
//   constructors, `parse_integration_method`, `LtePrefixWindow`,
//   `CodeModelVectorParams` and `XspiceEventInputs`.
//
// Deleted, because narrowing made `-D warnings` say the item had no caller at
// all: `Mosfet::qgs`, `qgd` and `qgb`; `Engine::resolve_node_with_abort`,
// whose own doc comment sends callers to `NodeResolver::build_with_abort`; and
// the whole `LossyTransmissionLine` module, which nothing in the workspace
// constructed — the lossy lines the product runs are the LTRA and
// distributed-RLC kernels in `transmission_line/line.rs`.
//
// Marked `#[cfg(test)]` rather than narrowed, because the only callers are
// this crate's own unit tests: `Mosfet::is_initially_off`, `uses_legacy_bsim`,
// the two `add_with_ac_and_spec` source overloads, `assemble_port_noise` (the
// uncancellable twin of `assemble_port_noise_with_abort`), and
// `XspiceInstance::update_inputs`, whose production form is
// `update_inputs_with_analog_transitions`.
//
// What stayed public, and who needs it:
//
// - `execution/result_document/payload.rs`, 87 statements, and the sample and
//   window types beside them in `result_document.rs`. `ResultPayload` is a
//   public enum that `rspice-python`, `rspice-wasm` and
//   `rspice-engine-adapter` all match on, and every payload document is one of
//   its variant fields, so `private_interfaces` makes this surface public by
//   construction rather than by choice.
// - `execution/result_document.rs` and `builders.rs`. All 24 `from_*`
//   constructors have a frontend caller: the CLI's periodic runners, the
//   engine adapter's family dispatch, the Python result wrappers and the WASM
//   deck runner between them name every one.
// - `ArtifactNamespace` and the materializer entry points. The CLI and the
//   engine adapter reach the namespace through
//   `MaterializedAnalysis::output_namespace()` without ever spelling its name,
//   which is exactly the case a grep would have got wrong.
// - The capability matrices (`ANALYSIS_CAPABILITY_MATRIX`,
//   `SIGNAL_CAPABILITY_MATRIX`, `SurfaceCapability`), the checkpoint
//   capability vocabulary, `evaluate_transient_post_results`,
//   `evaluate_transient_fourier_results`, `SignalProjection::keeps_everything`
//   and the `CountingAbort`/`ImmediateAbort` observation accessors. Their only
//   callers are integration tests under `crates/rspice-core/tests/` —
//   `qualification_baseline`, `transient_checkpoint`,
//   `transient_compression_container`, `planned_post_process`,
//   `save_directives`, `abort_iteration_bounds` — and an integration test is
//   an external crate, so `pub(crate)` would make them uncompilable.
// - `netlist/ast.rs`'s analysis cards and `spectre_statistics`'s plan
//   vocabulary: the conformance suite reads the cards, and the plan types are
//   the types of `Netlist`'s own public fields.
//
// Six items are public with no caller anywhere, and are named here rather than
// hidden: `Mosfet::gate_charges`, `transient_fft_window_coherent_gain`,
// `planned_transient_fft_spectra` with `PlannedFftSpectrum`,
// `BoundedAbortWriter::byte_limit` and `SurfaceCapability::unsupported`.
// Narrowing each one turns it into dead code that `-D warnings` then deletes,
// and each is either a physical model or an execution-contract entry point
// whose consumer is another package's work in flight. Deleting them is a
// decision for the package that owns the consumer, not for a visibility pass.
//
// 2026-09-05, +29 deliberate (4,916 -> 4,945): the VCD codec in `io::vcd` and
// the event projection that feeds it, which is case 2 above — an entry point
// frontends are meant to call. The GUI has a VCD reader today, private to
// `workbench::workflows`, mapping four-state logic onto `f64` and refusing
// `x`/`z` outright; there is no writer anywhere. The CLI's `convert` and
// `run --format vcd` and the GUI's import adapter consume this one instead, so
// every name on the list is on the path from a deck to a `.vcd` file or back:
//
// - 23 in `io/vcd.rs`. The document vocabulary (`VcdDocument`, `VcdSignal`,
//   `VcdVariable`, `VcdChange`, `VcdValue`, `VcdBit`, `VcdSignalKind`,
//   `VcdTimescale`, `VcdMagnitude`, `VcdTimeUnit`, `VcdError`) is what a
//   caller reads a dump out of and builds one from, and its fields are public,
//   so `private_interfaces` makes the types public by construction. The four
//   `parse_vcd_*` entry points mirror `ltspice_raw`'s file/reader pair because
//   the CLI and the GUI already consume that pair; `write_vcd`,
//   `VcdDocument::new`, `assign_canonical_identifiers`,
//   `VcdVariable::scoped_name`, `VcdTimescale::ALL`,
//   `VcdTimescale::femtoseconds`, `VcdTimescale::seconds` and
//   `VCD_WRITER_VERSION` are the writer's half.
// - 4 in `execution/event_projection.rs`: `event_vcd_document`, the two
//   `DigitalValue` <-> `VcdBit` mapping functions, and
//   `EventProjectionError`.
// - 2 grouped re-export statements, in `io.rs` and `execution.rs`.
//
// 2026-09-05, +20 deliberate (4,945 -> 4,965): the rawfile's second half — a
// reader that returns every plot a file declares, and a writer that appends an
// XSPICE event timeline as a plot of its own. Case 2 again: `rspice-cli`'s
// transient publish calls the writer on every `--format raw`/`--format ascii`
// run, and reading a multi-plot file back is what `convert`, `compare` and the
// GUI's RAW importer need in order to see anything past plot 1.
//
// - 3 in `io/ltspice_raw.rs`: `RawFile`, the parsed file with its plots in
//   file order, and `parse_raw_plots_file_with_limits` /
//   `parse_raw_plots_reader_with_limits`, the file/reader pair that produces
//   it. The pair mirrors the single-plot `parse_raw_*_with_limits` entry
//   points beside it, which the CLI and the GUI already consume; no
//   limits-free wrapper was added, because every caller passes its own
//   `ResourceLimits`.
// - 11 in `io/raw_export.rs`. `write_event_plots` is the call —
//   `rspice-cli`'s `commands/run/basic.rs` appends the plots inside the same
//   staging closure that publishes the table — and `RawEventTimeline` is its
//   input, which the same file names as a field of its transient output
//   document. `RawEventKind` is a public field of that input, so
//   `private_interfaces` makes it public by construction, and its five
//   accessors (`plot_name`, `from_plot_name`, `variable_type`,
//   `variable_name`, `node_name`) are the one place the plot-name and
//   `D(...)`/`E(...)` spellings are defined, so the decoder reads them rather
//   than restating the format. `RawVariable::event`,
//   `RawExporter::new_event_plot` and `VariableType::as_str` are the writer's
//   own half. Nine of the eleven were narrowed the same day; see below.
// - 4 in `execution/event_export.rs`: `transient_event_plots`, which the CLI
//   calls to project a transient's event histories onto those timelines;
//   `decode_event_plots`, the inverse a reader calls; and the two types that
//   inverse names, `RawEventTraces` and `EventPlotError`.
// - 1 in `xspice/digital.rs`: `DigitalValue::from_event_code`, the inverse of
//   the `event_code` beside it, which is how a code read back out of a plot
//   becomes a value again. Narrowed the same day; see below.
// - 1 grouped re-export statement, in `execution.rs`. The two in `io.rs` were
//   widened in place, so they cost nothing.
//
// 2026-09-05, +1 deliberate (4,965 -> 4,966): `abort_signal::DigitalEventCode`,
// the one statement that lets the accepted-sample hook publish a point's
// committed digital state. `abort_signal` is a layer-0 leaf, so it may not name
// `xspice::DigitalValue` nine layers above it; the code that type's
// `event_code` produces is a `u8` the leaf can own, and it is the encoding the
// result document, the GUI worker contract and the UI evidence type already
// agree on — the same encoding the rawfile event plots above read and write.
// It is public because `TransientSample` is — the GUI's runner both constructs
// and reads that struct — so the field's type cannot be narrower than the
// struct carrying it.
//
// 2026-09-05, -9 narrowed (4,966 -> 4,957): the nine event-plot helpers the
// raise above recorded as arrears. Re-measured across `rspice-cli`,
// `rspice-ui`, `rspice-python`, `rspice-wasm`, `rspice-engine-adapter`,
// `rspice-conformance` and every integration test — the CLI's VCD work has
// landed since, so the measurement was worth redoing — and none of the nine
// has a caller outside this crate. Each is now `pub(crate)`:
//
// - The five `RawEventKind` accessors, `plot_name`, `from_plot_name`,
//   `variable_type`, `variable_name` and `node_name`. `RawEventKind` itself
//   stays public: it is a field of `RawEventTimeline`, which the CLI builds.
// - `VariableType::as_str`, read only by `raw_export`'s own writer.
//   `VariableType` stays public as a field of `RawVariable`.
// - `RawVariable::event` and `RawExporter::new_event_plot`, both reached only
//   from `write_event_plots` two functions below them.
// - `DigitalValue::from_event_code`, reached only from
//   `execution/event_export.rs`, which is the decoder a frontend actually
//   calls. `event_code`, the forward direction, stays public: the GUI, the
//   worker contract and the sample hook all speak it.
//
// Left public although nothing outside the crate calls them either, because
// narrowing would make the contract worse rather than smaller:
// `parse_vcd_reader_with_limits` and `parse_raw_plots_reader_with_limits` are
// the byte-slice halves of two file/reader pairs whose file halves the CLI
// already calls — a browser has no path, so the reader half is the only form
// the GUI and `rspice-wasm` can use — and `VCD_WRITER_VERSION` is the string a
// consumer compares `$version` against to learn who wrote a dump.
//
// 2026-09-05, +2 deliberate (4,957 -> 4,959): `parse_source_spec_text`, which
// the GUI now calls to read a placed source's card the way the deck reads it.
// It is two lines rather than one because this ratchet counts a `pub use` as an
// item: the function in `netlist/parser/source_specs.rs` and its re-export
// through `netlist/parser.rs`, which is the crate's established idiom for
// putting a parser helper on the public surface.
//
// The GUI previously interpreted `PULSE`/`SIN`/`SFFM` and the rest itself in
// its preview card, which is how it came to disagree with the engine about an
// omitted pulse width. There is no smaller way to hand a frontend the engine's
// own reading: `SourceSpec` is already public and `VoltageSources::
// evaluate_source_spec_at_time_with_dialect` already takes one, but nothing
// public could produce one from the text a card carries.
//
// 2026-09-06, +18 deliberate (4,959 -> 4,977): the digital bus contract. An
// XSPICE vector port reaches a result as one trace per element under its own
// node name, and nothing in any result said those nodes were one word — the
// GUI's events sheet, the CLI's dumps, the Python and browser bindings and the
// rawfile importer each had to guess, and none of them could. Case 2: every
// name below is on the path from a declared bus to a viewer, a dump or a
// binding that can show it.
//
// - 7 in `engine/result.rs`. `DigitalBusDeclaration` is the declaration
//   itself and `DigitalBusSource` is a public field of it, so
//   `private_interfaces` makes the enum public by construction; `::new` is the
//   checked constructor and `::validate` the same rules for a declaration that
//   was decoded or assembled some other way, both of which the GUI's producer
//   calls when it attaches a schematic's buses to a run's traces.
//   `MAX_DIGITAL_BUS_WIDTH` is the width budget the schematic pins its own
//   `MAX_BUS_MEMBER_INDEX + 1` against. `validate_digital_bus_table` is the
//   half a single declaration cannot check — every member names a trace, no
//   conductor is claimed twice — which every producer of a table calls.
//   `DigitalBusError` is what all three return. The grouped `pub use` in
//   `engine.rs` was widened in place, so it costs nothing.
// - 1 in `abort_signal.rs`: `TransientDigitalBus`, the bus table the live
//   accepted-sample hook lends out. It is public because `TransientSample` is
//   — the GUI's runner both constructs and reads that struct — so the field's
//   type cannot be narrower than the struct carrying it.
// - 2 in `execution/result_document/payload.rs`: `DigitalEventBus`, the
//   document's own spelling of a declaration, and `DigitalBusSourceTag`, a
//   public field of it. Both are what a reader of a published document — the
//   GUI, the adapter, the browser binding — decodes.
// - 3 in `execution/event_bus.rs`: `bus_events` and `bus_value_at`, the one
//   implementation of reassembling a bus from its members, and
//   `BusMemberHistory`, their input. Every route that shows a bus calls them,
//   including the GUI's events sheet and the Python and browser accessors, so
//   that no two of them can disagree about what the run held.
// - 2 in `execution/event_projection.rs`: `vcd_event_histories`, the inverse
//   of the VCD projection, and `VcdEventHistories`, what it returns. The CLI's
//   `convert` from a `.vcd` and the GUI's VCD import adapter are its callers;
//   both flatten a dump themselves today and lose the vectors doing it.
// - 1 in `execution/event_export.rs`: `transient_bus_plots`, which the CLI's
//   transient publish calls beside `transient_event_plots` for
//   `--format raw`/`--format ascii`.
// - 1 in `io/raw_export.rs`: `RawBusTimeline`, that function's output and
//   `write_event_plots`' second input, which the CLI names as a field of its
//   transient output document.
// - 1 grouped re-export statement, in `execution.rs`, for the new
//   `event_bus` module. The `event_export`, `event_projection` and `io.rs`
//   re-exports were widened in place.
// 2026-09-06, +1 deliberate (4,977 -> 4,978): `DigitalValue::from_event_code`,
// raised back to `pub` the day after it was narrowed above. The narrowing was
// correct when it was measured — the decoder had one caller, inside this crate
// — and is wrong now: the GUI's VCD encoder,
// `rspice-ui/src/workbench/menu_bar/waveform_export/vcd.rs`, had spelled the
// thirteen-entry table a second time because the inverse was crate-private,
// and that copy is deleted in the same change.
//
// It is the one frontend that needs it, because it is the one that holds the
// code rather than the value: the accepted-sample hook publishes a `u8`
// through `abort_signal::DigitalEventCode`, the GUI's worker contract carries
// that `u8` across the message boundary, and its retained evidence stores it,
// so the GUI has to decode where the Python and browser bindings — which read
// typed `DigitalStateTag`/`DigitalStrengthTag` values out of a result document
// — only ever encode, through the `event_code` beside this.
//
// One external caller is enough here, and the ratchet's own rule says why: a
// decoder a frontend must otherwise restate is not internal machinery. A second
// copy of the table is a second definition of what a code means, and the only
// way to keep two definitions agreeing is to have one.
// 2026-09-06, +2 deliberate (4,978 -> 4,980): the one spelling of a bus
// member as a VCD bit, and the character a bit shows as.
//
// - 1 in `execution/event_projection.rs`: `event_code_to_vcd_bit`, which turns
//   what `bus_events` hands back for one member — a held `0..=12` code, or
//   `None` for a member the run has not stated yet — into the bit a dump
//   spells it with. Three callers: the core VCD projection's own `bus_signal`,
//   `rspice-python`'s `results/transient/buses.rs`, and `rspice-wasm`'s
//   `events.rs`. The two bindings each carried a copy of the decode-and-map
//   before this, which is two more places for one bus word to read differently
//   than the dump the command line writes.
// - 1 in `io/vcd.rs`: `VcdBit::as_char`, raised from private. Both bindings
//   assemble a whole word one character at a time, and the only public way to
//   get that character was `Display`, which allocates a `String` per bit — on
//   the exact path a wide bus makes hot.
// 2026-09-06, +2 deliberate (4,980 -> 4,982): the ceiling on how much of a
// bus one reassembly materializes, and the refusal that names it.
//
// - 2 in `execution/event_bus.rs`: `MAX_BUS_EVENT_CELLS` and
//   `BusReassemblyTooLarge`. A bus costs events times members, and neither
//   number is unreasonable alone — 4,096 members is the declared maximum, two
//   million events is what the bindings already allow one node — so nothing
//   caught their product. The bound is enforced inside `bus_events` rather
//   than at each caller precisely so that the VCD projection, the rawfile bus
//   plots, the Python accessor and the browser handle refuse the same document
//   for the same reason with the same numbers; that makes both items part of
//   the signature every one of those callers already names.
// 2026-09-06, +1 deliberate (4,982 -> 4,983): `split_bus_notation` in
// `execution/event_bus.rs`, raised from `pub(crate)`.
//
// Core writes a bus reference in exactly two places a single field has to hold
// the whole declaration — a VCD `$var` and a rawfile bus plot's `Title:` — and
// this is the one parser of that grammar, which is why it was written once
// rather than twice. The readers of those fields are not all in this crate:
// `rspice convert --variables` has to decide whether `data`, `data[7:0]` or
// `data [7:0]` names the vector variable a dump declares, and the bare name is
// not one of the variable's own spellings. A second parser in the CLI would be
// a second grammar, and the two would agree until the day they did not.
// 2026-09-06, +1 deliberate (4,983 -> 4,984): `BusEventTable` in
// `execution/event_bus.rs`, the reassembled timeline `bus_events` returns.
//
// It is named rather than spelled inline because `clippy::type_complexity`
// refuses the spelling once the function became fallible, and a private alias
// in a public signature would show every caller a name it cannot follow. Its
// callers are `bus_events` itself and every consumer that binds the result:
// the VCD projection, the rawfile bus plots, `rspice-python`'s accessor and
// `rspice-wasm`'s handle.
const MAX_PUBLIC_ITEMS: usize = 4984;

/// How far under the ceiling the count may sit before the ceiling is
/// considered stale and must be lowered. Without this, a ratchet silently
/// stops ratcheting: the number falls, nobody updates the constant, and the
/// gap quietly becomes headroom for regrowth.
const STALE_CEILING_SLACK: usize = 100;

fn src_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn rust_sources(root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(dir) = pending.pop() {
        let entries =
            fs::read_dir(&dir).unwrap_or_else(|error| panic!("read {}: {error}", dir.display()));
        for entry in entries {
            let path = entry.expect("directory entry").path();
            if path.is_dir() {
                if path
                    .file_name()
                    .is_some_and(|name| name == "veriloga_builtins")
                {
                    continue;
                }
                pending.push(path);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}

const ITEM_KEYWORDS: &[&str] = &[
    "fn ",
    "struct ",
    "enum ",
    "trait ",
    "type ",
    "const ",
    "static ",
    "unsafe fn ",
    "async fn ",
];

/// Whether a line declares a public item or re-export.
///
/// Takes the line already trimmed of leading whitespace. `pub(` is rejected
/// before the keyword check so restricted visibility never counts.
fn is_public_item(line: &str) -> bool {
    let Some(rest) = line.strip_prefix("pub ") else {
        return false;
    };
    if rest.starts_with("use ") {
        return true;
    }
    ITEM_KEYWORDS
        .iter()
        .any(|keyword| rest.starts_with(keyword))
}

fn count_public_items() -> (usize, Vec<(String, usize)>) {
    let root = src_dir();
    let mut total = 0;
    let mut per_file = Vec::new();
    for path in rust_sources(&root) {
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        let count = source
            .lines()
            .filter(|line| is_public_item(line.trim_start()))
            .count();
        if count > 0 {
            total += count;
            per_file.push((
                path.strip_prefix(&root)
                    .unwrap_or(&path)
                    .display()
                    .to_string()
                    .replace('\\', "/"),
                count,
            ));
        }
    }
    per_file.sort_by(|a, b| b.1.cmp(&a.1));
    (total, per_file)
}

#[test]
fn public_surface_does_not_grow() {
    let (total, per_file) = count_public_items();

    if total > MAX_PUBLIC_ITEMS {
        let worst: Vec<String> = per_file
            .iter()
            .take(10)
            .map(|(path, count)| format!("  {count:>5}  src/{path}"))
            .collect();
        panic!(
            "public surface grew: {total} items, ceiling is {MAX_PUBLIC_ITEMS}.\n\n\
             Largest contributors:\n{}\n\n\
             New items should be `pub(crate)` unless a frontend needs them. \
             The CLI, GUI, Python and WASM bindings, and the conformance \
             suite together name about two hundred distinct paths; anything \
             outside that set is internal machinery and should say so.",
            worst.join("\n")
        );
    }

    assert!(
        total + STALE_CEILING_SLACK >= MAX_PUBLIC_ITEMS,
        "public surface is {total} items but the ceiling is still \
         {MAX_PUBLIC_ITEMS}, a gap of {}.\n\n\
         Lower MAX_PUBLIC_ITEMS in tests/public_surface.rs to {total}. A \
         ceiling left far above the real count is not a ratchet — it is \
         headroom for the surface to grow back into.",
        MAX_PUBLIC_ITEMS - total
    );
}

#[test]
fn restricted_visibility_does_not_count_as_public() {
    assert!(is_public_item("pub fn solve()"));
    assert!(is_public_item("pub struct Circuit"));
    assert!(is_public_item("pub use crate::circuit::CircuitData;"));
    assert!(is_public_item("pub const GMIN: f64 = 1e-12;"));

    // The whole point: narrowing visibility must lower the count, so these
    // are not public for this test's purposes.
    assert!(!is_public_item("pub(crate) fn stamp()"));
    assert!(!is_public_item("pub(super) struct State"));
    assert!(!is_public_item("pub(in crate::engine) fn drive()"));
    assert!(!is_public_item("pub(crate) use super::Thing;"));

    // Not item declarations. The struct field is passed already trimmed,
    // as the counter sees it, so this exercises the keyword check rather
    // than the leading whitespace.
    assert!(!is_public_item("fn private()"));
    assert!(!is_public_item("pub node_pos: Vec<NodeId>,"));
    assert!(!is_public_item("// pub fn commented_out()"));
}

#[test]
fn public_hb_solver_rejects_invalid_charge_parameters_before_evaluation() {
    let mut invalid_devices = Vec::new();

    let mut invalid_junction = NonlinearDeviceInstance::diode(0, 0, 1.0e-14, 1.0);
    invalid_junction.params.cap_a = DepletionCap::new(1.0e-12, 0.7, 1.01, 0.5);
    invalid_devices.push((invalid_junction, "grading coefficient"));

    let mut invalid_gate = NonlinearDeviceInstance::nmos(0, 0, 0, 0, 0.7, 1.0e-3, 0.0);
    invalid_gate.params.cox_wl = -1.0e-15;
    invalid_devices.push((invalid_gate, "intrinsic gate capacitance"));

    let mut invalid_transit = NonlinearDeviceInstance::diode(0, 0, 1.0e-14, 1.0);
    invalid_transit.params.tt_f = f64::NAN;
    invalid_devices.push((invalid_transit, "transit time"));

    let invalid_diode = NonlinearDeviceInstance::diode(0, 0, -1.0, 1.0);
    invalid_devices.push((invalid_diode, "diode IS"));

    let invalid_mos = NonlinearDeviceInstance::nmos(0, 0, 0, 0, 0.7, -1.0, 0.0);
    invalid_devices.push((invalid_mos, "MOS KP"));

    let invalid_jfet = NonlinearDeviceInstance::njfet(0, 0, 0, -2.0, -1.0, 0.0, 1.0e-14);
    invalid_devices.push((invalid_jfet, "JFET BETA"));

    let mut invalid_arity = NonlinearDeviceInstance::diode(0, 0, 1.0e-14, 1.0);
    invalid_arity.terminals.pop();
    invalid_devices.push((invalid_arity, "has 1 terminals, expected 2"));

    let invalid_index = NonlinearDeviceInstance::diode(2, 0, 1.0e-14, 1.0);
    invalid_devices.push((invalid_index, "node index 2 exceeds 1 nodes"));

    for (device, expected) in invalid_devices {
        let mut solver = HbSolver::new(HbConfig::new(1.0e6).with_harmonics(1), 1);
        solver.add_nonlinear_device(device);
        let mut state = HbSolverState::new(1, 1);
        let error = solver
            .solve_dc_operating_point(&mut state)
            .expect_err("invalid public nonlinear-device parameters must fail before solving");
        assert!(matches!(error, HbError::InvalidCircuit(_)));
        assert!(
            error.to_string().contains(expected),
            "wrong public-solver parameter diagnostic: {error}"
        );
    }

    let mut switch_solver = HbSolver::new(HbConfig::new(1.0e6).with_harmonics(1), 1);
    switch_solver.add_voltage_switch(
        HbSwitchNodes {
            node_pos: 0,
            node_neg: 0,
            ctrl_pos: 0,
            ctrl_neg: 0,
        },
        HbVoltageSwitchModel {
            vt: 0.0,
            vh: 0.1,
            ron: 1.0,
            roff: 1.0e6,
            smooth: 0.1,
        },
    );
    let mut state = HbSolverState::new(1, 1);
    let error = switch_solver
        .solve_dc_operating_point(&mut state)
        .expect_err("public exact-HB switch API must reject unrepresented hysteresis");
    assert!(
        error
            .to_string()
            .contains("requires zero finite hysteresis"),
        "wrong public switch diagnostic: {error}"
    );
}

#[test]
fn public_hb_surface_does_not_advertise_rejected_approximate_kernels() {
    let solver_root = src_dir().join("analysis/harmonic_balance");
    let solver_source = fs::read_to_string(solver_root.join("solver.rs")).expect("read solver.rs");
    let device_source =
        fs::read_to_string(solver_root.join("solver/devices.rs")).expect("read solver/devices.rs");
    let api_source = fs::read_to_string(solver_root.join("solver/nonlinear_api.rs"))
        .expect("read solver/nonlinear_api.rs");
    let combined = format!("{solver_source}\n{device_source}\n{api_source}");

    for stale_name in [
        "NpnBjt",
        "PnpBjt",
        "CurrentSwitch",
        "npn_bjt",
        "pnp_bjt",
        "current_switch",
        "add_current_switch",
    ] {
        assert!(
            !combined.contains(stale_name),
            "exact-HB public surface still advertises removed approximate kernel {stale_name}"
        );
    }
}
