//! Ratchet the core's public item declarations against reviewed frontend needs.
//!
//! New implementation helpers should use restricted visibility. Public APIs
//! should serve a frontend contract: CLI, Studio, Python, WASM, or conformance.
//! Raising the ceiling requires explaining the intended caller in the commit,
//! or showing that a move preserves public names and only adds re-exports.
//!
//! This is a source-level proxy, not a census of reachable exported names:
//! it counts public item declarations and re-export statements at the start
//! of a line after indentation. A grouped re-export counts once, and items
//! inside private modules still count. Fields and restricted visibility do not,
//! and neither does a module the family moved into another of its crates and
//! re-exports whole at its old path: that line is the module's declaration.
//! Generated `veriloga_builtins` trees are excluded. The keyword list below
//! defines the exact count; moving declarations can change it without adding API.
//!
//! Keep the current qualification here; older decisions remain in Git history.

use std::fs;
use std::path::{Path, PathBuf};

#[path = "common/family_src.rs"]
mod family_src;

use family_src::{family_relative, family_src_roots, read_family_src};
use rspice_core::analysis::harmonic_balance::{
    DepletionCap, HbConfig, HbError, HbSolver, HbSolverState, HbSwitchNodes, HbVoltageSwitchModel,
    NonlinearDeviceInstance,
};

/// Current audited public-statement count, without growth headroom.
///
/// Reconciled from 5,468 to 5,534 after narrowing 27 implementation statements
/// in envelope numerics, BSIM AC stamping, line histories and charge projection.
/// The remaining net growth since the last reconciled baseline serves these
/// deliberate cross-crate contracts (some additions are offset by removals):
///
/// - Prepared spectral-envelope configuration, state and mission execution:
///   `rspice-simulation-contract::envelope_multirate` and the simulation
///   service's `engine_services::envelope_fourier::multirate` driver.
/// - Authored DC/frequency-table execution and QPSS/QPAC/QPXF/QPNOISE payloads:
///   CLI run commands, Python/WASM deck execution, and simulation results.
/// - Typed elaboration/compiler diagnostics and input-data dependencies:
///   CLI check/run, Python errors, simulation model compilation and XSPICE.
/// - Bounded RAW/VCD readers, table-unit metadata and exact tick conversion:
///   CLI waveform import, clipping, conversion and result comparison.
/// - Retained impulses, correlated noise and observation vocabulary:
///   Python results, simulation transport and result-document consumers.
/// - Model-catalog matching and authored-analysis options:
///   CLI models/run and the shared frontend execution contracts.
/// - Seven frequency-table document statements retain CLI/WASM AC/noise row
///   coordinates: two builders, two metadata types, an accessor, a re-export,
///   and the coordinate-unit resolver shared with control presentation.
/// - Borrowed frequency-table admission shared by CLI check/run preflight
///   and the core executor.
///
/// Changes to this ceiling must continue to identify their frontend caller;
/// this accounting does not exempt any family from the ratchet.
// Two bounded/cancellable table projection entry points serve CLI/WASM
// callers without forcing the default numerical budget on their output.
// One text-provenance writer lets CLI RAW exports preserve exact table labels.
// Transfer gain units are retained for typed documents and control consumers.
// An explicit coordinate-layout writer and reader disambiguate CLI table exports
// from legacy operating-point, FFT and event carriers that share display titles.
// PRINT retains nonfinite scalar determinations and their order for CLI/WASM.
// One opaque metadata writer serves CLI FFT RAW publication via inert Options.
// Bounded continuous measurement entry points and their relocated re-exports
// serve CLI run/basic.rs (DC/TRAN), run/frequency.rs (AC/NOISE), and the core
// transient post-processor. Reconciled their seven added source declarations
// against the prior five-item gap. Four further AC/noise table measurement
// adapters (scalar and continuous) serve CLI run/frequency.rs while keeping
// resolved parameter snapshots internal. The census was 5,560, with no slack.
// The 7-9 October checkpoint, stability and observation work added 111 public
// statements and removed 14; two serde helpers are narrowed again. The rest
// have callers outside the crate:
// - rspice-simulation: streamed transient checkpoints (start, stream,
//   observer, request validation, runner), transient event changes, impulse
//   trace views and Fourier impulse analysis, stability margins and pole
//   spectra, control analysis classification, output-schedule errors.
// - rspice-cli: control analysis naming and support checks, Touchstone
//   versions, numeric JSON documents and decimal rounding checks, RAW axis and
//   coordinate validation, circuit storage re-exports, circuit-pole evidence.
// - rspice-python, rspice-results, rspice-formats, rspice-engine-adapter and
//   rspice-simulation-contract: stability verdicts and assessments, pole
//   spectra, voltage impulse traces, Monte Carlo documents with units, the
//   document version, and frequency-table parameter names.
// - The XSPICE code-model API: analog input thresholds and their behaviour,
//   rational transfer descriptors, and committed/other digital drivers.
// - Core integration tests: scheduler checkpoints, pole-spectrum runners,
//   mixed observer/bridge controls and small-signal descriptors.
// The census was 5,655, with no slack.
// Four structured-refusal statements: the `for_analysis_kind`, `for_instance`
// and `for_module` builders on `UnsupportedCapabilityError` and the
// `SimulationError::instance_name` accessor, read by the CLI error payload
// (`rspice-cli` cli/error.rs), the Python error attributes
// (`rspice-python` errors/simulation.rs) and the typed simulation refusal
// (`rspice-simulation` error.rs). The census was 5,659, with no slack.
// Moving the result-document scalars below `engine` (so the control host
// stops reaching up into `execution`) keeps every public name and adds two
// statements: the re-export under `execution::result_document`, which
// rspice-cli (compare determinations, control output, Monte Carlo waveform
// IO) and the engine-adapter
// tests import through, and `ResultScalarError`, the error
// `ResultScalar::new` returns now that the constructor cannot name a
// result-document error. The census was 5,661, with no slack.
// Sixteen statements export the family-by-analysis capability matrix
// (`engine::analysis_capability`, re-exported from `engine`): the
// `CapabilityFamily`, `CapabilityAnalysis`, `InstanceGate` and `CellSupport`
// types with their `ALL` inventories and `tag`/`label`/`kind`/`status`
// accessors, `capability_cell` and `capability_matrix`. The design's
// intended callers are the frontends' support tables (the application's
// analysis feature tables and the CLI capabilities document); today's caller
// is the core integration suite that runs every cell on a fixture of its
// family (`tests/capability_refusals.rs`).
// The census was 5,677, with no slack.
// One statement for `.ESAVE` lists: `netlist::EventSaveSelection`, the type of
// the public `SimulationOptions::xspice_event_trace_save` field, which the
// conformance runner's Xyce option contracts read and every frontend that
// builds options names. The census was 5,678, with no slack.
// Seventeen statements for HDL runs (design P2 lane L1), after narrowing
// eight that have no caller outside the core:
// - `config`: the output record vocabulary `HdlOutputRecord`, `HdlPoint`,
//   `HdlStream`, `HdlOrigin`, `HdlSeverity` and `HdlSeverity::label`, which
//   the CLI console renders (`rspice-cli` console.rs) and the headless
//   transient result carries (`rspice-simulation` results.rs `hdl_output`);
//   the `HdlOutputSink` trait the CLI console and the simulation bridge's
//   capture implement; the bounded `HdlOutputCollector` with `new` and `take`,
//   which the simulation bridge collects a transient's output with; and
//   `HdlRunConfig` and `HdlDelaySelect`, the types of the public
//   `SimulationConfig::hdl` field and its `delay_select`, which the CLI's run
//   flags set.
// - `xspice::verilog`: `run_digital_testbench` with `HdlTestbenchOptions`,
//   `HdlSchedule`, `HdlTestbenchEnd` and `HdlTestbenchReport`, the design's
//   self-checking testbench entry point (P2 section 2.3) for the ivtest
//   conformance harness; today the core's testbench suite calls it, and
//   restricted visibility would leave it dead code in a non-test build.
// The census was 5,695, with no slack.
// Three statements for the typed failure of an operating-point convergence
// aid that stops short of the circuit's own operating point:
// `ContinuationFailure`, carried by `SimulationError::ContinuationFailed`,
// and the `ContinuationAid` and `ContinuationStage` it names. Enum variant
// payloads must be public; the simulation bridge (`rspice-simulation`
// error.rs) and the Python exception mapping (`rspice-python`
// errors/simulation.rs) read the variant. The census was 5,695 under a
// ceiling of 5,698.
// Small-signal analyses around a transient state (`ACTIMES=`/`CHECKPOINT=`)
// add eight statements for the CLI run command and the simulation service:
// - `engine`: `run_small_signal_at_transient_times` and
//   `run_small_signal_at_checkpoint`, which both call; their
//   `StateLinearizedResult` and its `SmallSignalCardResult`, whose variants
//   both match; and the one re-export that names the two types outside the
//   private module.
// - `netlist`: `SmallSignalStateRequest` and `SmallSignalStateSource`, the
//   types of the public `Netlist::small_signal_states` field, which the
//   simulation service's headless tasks match and both callers' state
//   routing matches; and `Netlist::small_signal_state`, the request of one
//   card, which the CLI reads to route the card and the simulation service to
//   run or refuse it.
// The census was 5,703.
// `netlist::lexer::is_xyce_device_name` adds one: the design crate's DRC
// extraction (`drc/extraction.rs`) judges a card's name by the parser's own
// rule rather than a copy of it. The census was 5,704.
// Twenty-four statements came in with the Console and the Problems page
// together (a `pub mod` line is not counted):
// - The log scope that carries a run onto the engine's worker threads, eight,
//   for the simulation service's run log (`rspice-simulation` engine_log.rs):
//   `LogScope` with `new`, which the run's sink is installed through, `owner`
//   and `point`, which every offered record is read through, and `at_point`,
//   which its worker test labels a sample with; `capture` and `enter`; and
//   `LogScopeGuard`, which `enter` returns.
// - `LogScope::collecting`, one: the run sink keeps the run's notice
//   collector in that scope, so a notice raised on a worker reaches the run
//   that spawned it.
// - The notices a run raises and completes with, nine, read by the run sink
//   and the Problems page: `notice::EngineNotice` and the three types its
//   variants carry (`ParameterReplacement`, `ParameterScope`,
//   `TimestepFloor`); the `RecordedNotice` a run delivers; and the
//   `NoticeCollector` with `new`, `install` and `take`.
// - The engine refusals the Problems page states before a run, five, read by
//   the simulation service's preflight (`rspice-simulation`
//   execution/snapshot/engine_refusals.rs): `DcBranchVoltageLoop`, its
//   re-export from `introspection` and `Circuit::dc_branch_voltage_loops`;
//   `ModelBinCoverageGap` and the builder's `model_bin_coverage_gaps`.
// - `ParameterRedefinitionPolicy::binding_definition`, one, a method of a
//   public policy type that the parser and the source map read in this crate.
// The census was 5,728, with no slack.
// Five statements let the netlist editor read a deck the way the parser read
// it, for the application's value hover and reference lint
// (`rspice-app` netlist_document language.rs and diagnostics.rs):
// `UnreadSourceRegion` and `UnreadSourceReason`, the lines the parser skipped
// and why; `Netlist::unread_root_source`, the ones in the deck's own text;
// `source_map_for_editor_as_read`, the editor map with them applied; and
// `NetlistSourceMap::unread_definition_for`, where a name nothing read binds
// is written. The census was 5,733, with no slack.
// `netlist::spell_ground_probe_arguments` adds one: a schematic run's
// preparation (`rspice-simulation` analysis_preparation) writes the drawn
// ground's names as node 0 inside the cards and expressions it adds to the
// deck by the parser's own rule for where a node argument stands, rather than
// a copy of it. The census was 5,734, with no slack.
// Two statements keep an execution's installed Verilog-A from eviction until
// it is done with it: `hold_project_veriloga_sources_for_session` and the
// `HeldVerilogASources` it returns, which the simulation service's prepared
// runtime sets install through (`rspice-simulation` veriloga.rs, behind the
// runner, preflight's deck builds and checkpoint routing).
// `IncludeProcessor::process_sealed_root_reading_veriloga` adds one: the
// simulation service's model sources (`rspice-simulation` model_sources.rs)
// read the Verilog-A directives the library sections a process selects carry,
// each with the file that wrote it, so a corner's deck installs its own
// sections' runtimes and never another section's.
// The census was 5,737, with no slack.
// Two statements are removed: `register_project_veriloga_sources_for_session`
// and its `_with_limits` form, which installed sealed sources without holding
// them. Nothing outside the core called them, and a caller that did would
// leave what it installed for the next install to evict before it built;
// the core's integration tests install through the hold instead.
// The census was 5,735, with no slack.
// Three more are removed for the same reason:
// `register_project_veriloga_runtimes_for_session`, its `_with_limits` form
// and the one-runtime `register_project_veriloga_runtime_for_session`, which
// installed sealed runtimes without holding them and had no caller outside
// the core. `hold_project_veriloga_sources_for_session` is now the only way
// to install a sealed source.
// The census was 5,732, with no slack.
// `log_scope::logged` adds one: the simulation service's run log
// (`rspice-simulation` engine_log.rs) says which line it kept for each engine
// record it is offered, so the engine site that logged a warning raises the
// notice it states with that line. The census was 5,733, with no slack.
// `ContinuationAid` moved down into `notice`, whose convergence notice carries
// it, so the device models that record notices do not reach up into
// `analysis` through it. The move adds one re-export and no name:
// `analysis::error` re-exports the aid for the `ContinuationFailure` beside
// it, and every public path to it is unchanged. The census was 5,734, with no
// slack.
// `ModelBinCardGeometry::contains` adds one: the model library
// (`rspice-model-library` bin_family.rs) names the bin a component's L and W
// select for the Properties model picker by the containment test circuit
// construction applies, rather than by a copy of its tolerance. The census is
// 5,735, with no slack.
// `HarmonicComponent::sine_referenced_phase` adds one: the CLI `.four` table
// (`rspice-cli` fourier_document.rs) reports SPICE's sine-referenced phase
// from the same conversion the result document applies. Deleting
// `FourierAnalysis::analyze_current_with_abort`, which nothing called (current
// traces go through `analyze_impulses_with_abort`), takes one away. The census
// stays 5,735, with no slack.
// `NetlistSourceLocation` and `UnsupportedCapabilityError` moved down into
// `source_location` and `capability`, below the configuration and XSPICE code
// that name them, so neither reaches up into `netlist` or `analysis` for them.
// The moves add one re-export and no name: `netlist` re-exports the location,
// and `analysis::error` re-exports the refusal in the statement that already
// re-exported `ContinuationAid`. Every public path to both is unchanged. The
// census is 5,736, with no slack.
// The `ModelBin*` geometry-bin types moved down from `engine::builder` into
// `netlist::model_bin`, so a reader of a card's bin bounds
// (`rspice-model-library` bin_family.rs and facts.rs) need not depend on the
// engine. The move adds one grouped re-export in `model_resolution` and no
// name; every `rspice_core::engine::ModelBin*` path is unchanged. The census
// is 5,737, with no slack.
// `log_scope::canonical_target` adds one: the run log (`rspice-simulation`
// engine_log.rs), the CLI's JSON log and the application's engine log name a
// record by its `rspice_core` target whichever crate of the family emitted it,
// so the console and `RUST_LOG` directives keep today's names after the split.
// The census is 5,738, with no slack.
// One transient for a `.TRAN` card and the `ACTIMES=` cards that share it adds
// three, each called by rspice-simulation: `TransientState`, the state a
// `.TRAN` task's run captures and its card's task reads
// (engine_services/transient_state.rs); `Engine::run_tran_capturing_states_to_stop`,
// that run (engine_bridge/transient.rs); and
// `Engine::run_small_signal_at_transient_state`, the card around the captured
// state (engine_services/transient_state.rs). The census is 5,741, with no
// slack.
// Two hidden statements let every crate of the split family compare its
// feature flags with the crates below it at compile time: the
// `family_features::FamilyFeatures` struct and the `FAMILY_FEATURES` constant
// the `family_features!` macro declares at a crate's root. A crate above reads
// both through the macro, so neither can be restricted. The census was 5,743,
// with no slack.
// The converter insertion record adds seven: `ConverterInsertion` and its
// field types `ConverterKind`, `ConverterSource`, `ConverterSupply` and
// `ConverterParameter`, the `circuit` re-export that names them, and
// `CircuitData::converter_insertions`. The CLI's `check --converters`
// (`rspice-cli` commands/check.rs) lists and emits the records a build
// inserted. The census is 5,750, with no slack.
// `notice::BoundarySupplyKind` adds one, for the `EngineNotice::BoundarySupply`
// variant it classifies: the workbench's Problems list (`rspice-app`
// diagnostic_store/run_notices.rs) raises one row per boundary whose supply
// was inferred, defaulted or held static by a template, naming the net. The
// census was 5,751, with no slack.
// Hidden items now count apart, under `MAX_HIDDEN_FAMILY_ITEMS`, so the two
// `family_features` statements leave this census. The census was 5,749, with
// no slack.
// `ResourceLimits::with` adds one: the limits are non-exhaustive, so a crate
// other than their own cannot override a field with struct-update syntax, and
// every override in the family spells it this one way. The census was 5,750,
// with no slack.
// `AnalysisResultKind` moved down into `identity`, beside the `AnalysisKind`
// whose `result_kind` names the family each analysis produces, so that match
// stays exhaustive in the enum's own crate. The move adds two:
// `AnalysisKind::result_kind`, and the re-export that keeps
// `execution::capability::AnalysisResultKind` and every path through it.
// `analysis_result_kind` stays and delegates. The census was 5,752, with no
// slack.
// `netlist::WrealResolution` adds one: the type of the public
// `SimulationOptions::wreal_resolution` field `.options wreal_resolution`
// sets, which the builder hands the Verilog-A compiler for every deck source
// (engine/builder/veriloga_sources.rs). The census was 5,753, with no slack.
// `io::vcd::VcdDumpOff` adds one: the element type of the public
// `VcdDocument::dump_off`, which the `d_vcd` digital stimulus reads to refuse
// replaying a stretch the dump did not record. The census was 5,754, with no
// slack.
// `Engine::code_model_data_files_with_abort` and the `engine::CodeModelDataFile`
// it returns add two: rspice-simulation's `data_files` seals the file every
// code-model data-file parameter names into a prepared run, registering its
// bytes under the path the model reads, so a prepared run, the app and the
// browser worker replay `d_source`, `d_vcd` and the table models without a
// file system. The census was 5,756, with no slack.
// `engine::hdl_external_cells` adds one: the table of a deck's cells its HDL
// may instantiate as SPICE leaves, which a caller compiling the deck's HDL
// ahead of the build (the conformance harness's sealed project runtimes)
// hands the Verilog-A compiler, as the builder does. The census was 5,757.
// Deleting public items nothing used brings it to 5,685.
// The `.ASSERT` card adds six: `netlist::AssertStatement` (the element type of
// the public `Netlist::assertions`) with the `AssertCheck`, `AssertTarget`,
// `AssertLevel` and `AssertAnalysis` its fields are typed by, and the
// re-export that names them under `netlist`. The assertion evaluator
// (`analysis::assertions`) reads all five, and the CLI's report and exit
// status read the level. The census was 5,691, with no slack.
// The assertion evaluator adds five: `analysis::AssertionReport` and the
// `AssertViolation` it lists, which `TransientPostResults::assertions` and
// rspice-results' saved checks carry, and `evaluate_tran_assertions`,
// `evaluate_op_assertions` and `evaluate_dc_assertions`, which the CLI calls
// after each analysis (the transient one also from the post-results pass).
// They join the existing `analysis` re-export. The census was 5,696.
// Deleting public items nothing used brings it to 5,446, with no slack.
// `engine::hdl_delay_select` adds one: the min:typ:max selection a run's
// resolved configuration gives its HDL, in the compiler's spelling, which
// rspice-simulation compiles a project's sealed Verilog-A runtime under, and
// registers it under, so a project design runs at the deck's selection as
// the builder compiles a deck's own sources. The census is 5,447, with no
// slack.
// The HDL source map adds six: `TransientResult::hdl_sources`' element type
// `HdlTraceSource`, with the `HdlSourceLine`, `HdlTraceDriver` and
// `HdlDriverKind` its fields are typed by, and the result document's
// spelling of them (`HdlSourceDocument` and the types it holds), which the
// transient payload's `hdlSources` carries to the app and the bindings. The
// census is 5,453, with no slack.
// `engine::TransientEnd` and its `into_checkpoint` add two: how a
// checkpointed or resumed transient ended, at its stop time with the
// checkpoint that extends it or at a model's `$finish`, `$stop` or `$fatal`
// with none, which `Engine::run_tran_checkpointed` and `run_tran_resume`
// return and the CLI's `--checkpoint`, the Python bindings and the PSS and HB
// continuations read. The census is 5,455, with no slack.
// One civil calendar and one rawfile writer add five, each replacing private
// copies in the frontends: `time_compat::civil_from_unix_days`, which the
// hardcopy timestamp and the app's licence dates call, and `asctime_utc`, the
// Python binding's raw `Date:`; `raw_export::write_raw_plot`, its
// `RawPlotLayout` and `write_raw_exponent`, through which the CLI table
// export and the Python binding write their real and complex plots. The
// census is 5,460, with no slack.
// The HSPICE `.VEC` card adds three: `netlist::VectorFileReference` (its
// struct and its re-export), the card that `Netlist::vector_files` lists and
// the builder and the assertion evaluator read the file through, and
// `netlist::AssertSample`, one instant of the sampled check
// (`AssertCheck::Sampled`) a vector file's expected output becomes. The
// census is 5,463, with no slack.
// Deleting public items nothing used brings it to 5,313, with no slack.
const MAX_PUBLIC_ITEMS: usize = 5313;

/// The `#[doc(hidden)] pub` items of the family, counted apart from
/// `MAX_PUBLIC_ITEMS`. A hidden item is plumbing that one crate of the engine
/// family reaches in another: it is outside the documented API, so it never
/// raises that ceiling, but it is still a public path any crate can name, so
/// it has a falling ceiling of its own. An item leaves this count when its
/// last reach across a crate boundary goes, and then returns to restricted
/// visibility.
// The engine reaches its numerics, expression, notice, log-scope, resource
// and result-scalar plumbing through hidden items so that those modules can
// move into a crate of their own below it, and every crate of the family
// reads the two `family_features` statements. The census was 349, with no
// slack.
// `log_scope::warn_line` adds one: the macro is exported so that every crate
// of the family logs through the one definition. The census was 350, with no
// slack.
// `AbortReason::select`, `AnalysisKind::control_command`,
// `AnalysisKind::from_tag` and `ResultScalarError::fold` add four: a match on
// a non-exhaustive base enum is exhaustive only in the enum's own crate, so
// each moved beside its enum and the engine calls it there. The census was 354,
// with no slack.
// A field that carries an invariant is never public, hidden or not, since any
// crate could then break it: `ResultScalar`, `QualifiedOrdinarySpectrum`,
// `FiniteDynamics` and `ExactElimination` keep theirs private to their crate
// and add fifteen hidden getters and checked writers in their place. The
// census was 369, with no slack.
// `expr::Context::at_time` adds one: the context's logarithm domain stays
// private to its crate, so the behavioral sources above it move a context to
// another time through the method rather than struct-update syntax. The
// census was 370, with no slack.
// `OpLabel::generated` adds one: the compiled Verilog-A catalog that alone may
// mint such a label lives in the engine, above the label's crate. The census
// was 371, with no slack.
// `notice::boundary_supply` adds one: the engine's boundary-converter builder
// records an inferred or defaulted supply through it, like the other notice
// writers above it. The census was 372, with no slack.
// `DcMatchCard::voltage_probe` adds one: the engine's `.DCMATCH` tests state a
// card through it, and they sit above the parser's crate, so it compiles under
// `test-support` as well as `cfg(test)`. The census was 373, with no slack.
// `SweepPointGenerationError::fold` adds one: the sweep failure is
// non-exhaustive and moves with the parser below the engine, so the match
// that names each failure moved beside its enum, and the DC sweep, `.STEP`
// and the deck planner convert through it. The census was 374, with no slack.
// The engine reaches its parser, expression-context, output-request,
// measurement, Spectre-statistics, topology and XSPICE-parameter plumbing,
// and the control protocol, through hidden items so that the netlist,
// library and control-protocol modules can move into a crate of their own
// below it. `Netlist`, `OutputNodeNamespace`, `OutputRequest` and
// `MeasureExpression` keep their invariant-carrying fields private and add
// hidden getters and narrow writers in their place. The census was 536, with
// no slack.
// `CircuitData::voltage_source_storage` and `CircuitData::current_source_storage`
// add two: the parser's checks that read a built circuit's independent sources
// run from an integration test, outside the crate, so the parser's own unit
// tests never reach up to the engine. The census was 538, with no slack.
// The circuit store, engine and analyses reach the device models' stamping,
// charge, noise, checkpoint, periodic and continuation plumbing through hidden
// items so that the device models can move into a crate of their own below
// them. Checkpoint carriers and computed records keep public fields; device
// instances, `CplBranchCurrents` and `LtraRgTwoPort` keep theirs private and
// add hidden getters and narrow writers in their place. Five test helpers and
// the two BSIM oracle model cards compile under `test-support` as well as
// `cfg(test)`, for the engine's tests above them. The census was 1,225, with no
// slack.
// The generated Verilog-A device set's analysis, checkpoint, rollback, noise
// and charge plumbing, the switch and TEAM memristor checkpoints, the BJT
// charge linearization, the line responses and the prescribed integral rates
// add 86 more for the same move. Their stamp records and charge records keep
// public fields.
// `BuiltinVerilogADevices::operating_point_analysis_override` adds one, under
// `test-support` as well as `cfg(test)`: the circuit's test that an analysis
// start reaches its generated instances reads it, from above the device
// models.
// `CircuitData::bsim4v8_devices`, `CircuitData::b3soi_dd_devices` and
// `Bsim4v8Device::eval_gmin` add three: the BSIM4 charge-companion and B3SOIDD
// stamp-derivative checks build their instances through `Engine`, so they run
// from an integration test, outside the crate, and read the built instances
// back through these. The census was 1,315, with no slack.
// `numerics::integration::code_model_coincidence` and
// `BreakpointManager::merge_tolerance` add two: the transient engine tells
// every code model the window it counts as the current instant, which the
// breakpoint schedule's merge tolerance and the solver floor set, and both
// live in the base crate below it. The census was 1,317, with no slack.
// The HSPICE `.VEC` reader adds seven: it sits with the parser, below the
// engine, and the circuit build and the assertion evaluator read a card's
// vector file through `hspice_vec::parse` and the `VectorFile` it returns,
// refuse what RSpice does not realize through `VectorFile::check_realizable`,
// drive the inputs through `VectorFile::input_sources`, check the outputs
// through `VectorFile::output_checks`, report under
// `VectorFileReference::report_name`, and name the line a refusal comes from
// through `VectorFileError`. The census was 1,324, with no slack.
// The settle-cost counters add six, so that the XSPICE code models can move
// into a crate of their own below the engine: the circuit's settle loop bumps
// two of them, and the counter type, its event-world sum, `reset` and
// `counts` are what the engine's settle ratchets read, so they compile under
// `test-support` as well as `cfg(test)`. The census was 1,330, with no slack.
// The event-queue checkpoint adds sixteen for the same move: the circuit's
// event checkpoint captures and restores through its re-export, its three
// records and their constructors, and the event queue, its values, their two
// shared cells and the scheduler's slot id that those signatures name. The
// re-export, its module and the three scheduler and trace readers it calls
// compile under `test-support` as well as `veriloga` and `cfg(test)`, for the
// circuit's checkpoint tests. The census was 1,346, with no slack.
// The circuit store, the engine's builder and the analyses reach the event
// queue and its values, the trace journal, resource transactions, data-file
// readers, auto-bridge names, the Verilog host's format, effect, coupling and
// checkpoint records, and the mixed-signal coordinator's boundary, trace and
// checkpoint plumbing through hidden items for the same move, with each type
// their signatures name. The event scheduler's `schedule`, `len` and
// `is_empty` compile under `test-support` as well as `cfg(test)`, for the
// engine's event tests. The census is 1,598, with no slack.
const MAX_HIDDEN_FAMILY_ITEMS: usize = 1598;

/// How far under the ceiling the count may sit before the ceiling is
/// considered stale and must be lowered. Without this, a ratchet silently
/// stops ratcheting: the number falls, nobody updates the constant, and the
/// gap quietly becomes headroom for regrowth.
const STALE_CEILING_SLACK: usize = 100;

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
        return family_module_reexport(line).is_none();
    }
    ITEM_KEYWORDS
        .iter()
        .any(|keyword| rest.starts_with(keyword))
}

/// The crate directory and module a `pub use rspice_core_<name>::<module>;`
/// line names: a module of the family re-exported whole at the path it had
/// before it moved into another crate of the family. A module that keeps a
/// child of its own in this crate is declared inline instead, and its
/// `pub use rspice_core_<name>::<module>::*;` re-exports the moved module
/// whole in the same way.
///
/// That statement is the module's declaration, as the `pub mod` it replaced
/// was, and counts as `pub mod` does: not at all. The module's items are
/// counted in the crate that writes them, so a move adds nothing here.
/// [`every_family_module_reexport_names_a_module_of_that_crate`] holds each
/// such line to a module that crate declares, so a function or type
/// re-exported the same way cannot pass for one.
fn family_module_reexport(line: &str) -> Option<(String, &str)> {
    let path = line
        .strip_prefix("pub use rspice_core_")?
        .strip_suffix(';')?;
    let path = path.strip_suffix("::*").unwrap_or(path);
    let (name, module) = path.split_once("::")?;
    let snake = |word: &str| {
        !word.is_empty()
            && word
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    };
    (snake(name) && snake(module))
        .then(|| (format!("rspice-core-{}", name.replace('_', "-")), module))
}

/// Whether the item declared on `lines[index]` carries `#[doc(hidden)]`
/// among the attributes and doc comments directly above it.
fn is_hidden_item(lines: &[&str], index: usize) -> bool {
    lines[..index]
        .iter()
        .rev()
        .map(|line| line.trim_start())
        .take_while(|line| line.starts_with("#[") || line.starts_with("///"))
        .any(|line| line == "#[doc(hidden)]")
}

/// The public statements of one source, split into those the API documents
/// and those `#[doc(hidden)]` keeps out of it.
fn count_source(source: &str) -> (usize, usize) {
    let lines: Vec<&str> = source.lines().collect();
    let mut public = 0;
    let mut hidden = 0;
    for (index, line) in lines.iter().enumerate() {
        if is_public_item(line.trim_start()) {
            if is_hidden_item(&lines, index) {
                hidden += 1;
            } else {
                public += 1;
            }
        }
    }
    (public, hidden)
}

/// The count over every crate of the `rspice-core` family, the hidden count
/// beside it, and each file's share of the first. Every crate declares
/// something public, so a root that counts nothing was not read and fails
/// here.
fn count_public_items() -> (usize, usize, Vec<(String, usize)>) {
    let mut total = 0;
    let mut hidden_total = 0;
    let mut per_file = Vec::new();
    for root in family_src_roots() {
        let mut root_total = 0;
        for path in rust_sources(&root) {
            let source = fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
            let (count, hidden) = count_source(&source);
            hidden_total += hidden;
            if count > 0 {
                root_total += count;
                per_file.push((family_relative(&path), count));
            }
        }
        assert!(
            root_total > 0,
            "{} declares no public item; the scan read nothing there",
            root.display()
        );
        total += root_total;
    }
    per_file.sort_by(|a, b| b.1.cmp(&a.1));
    (total, hidden_total, per_file)
}

#[test]
fn public_surface_does_not_grow() {
    let (total, _, per_file) = count_public_items();

    if total > MAX_PUBLIC_ITEMS {
        let worst: Vec<String> = per_file
            .iter()
            .take(10)
            .map(|(path, count)| format!("  {count:>5}  {path}"))
            .collect();
        panic!(
            "public surface grew: {total} items, ceiling is {MAX_PUBLIC_ITEMS}.\n\n\
             Largest contributors:\n{}\n\n\
             New items should be `pub(crate)` unless a frontend needs them. \
             Identify the intended frontend caller before raising the ceiling; \
             implementation machinery should use restricted visibility.",
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
fn hidden_family_surface_does_not_grow() {
    let (_, hidden, _) = count_public_items();

    assert!(
        hidden <= MAX_HIDDEN_FAMILY_ITEMS,
        "the `#[doc(hidden)]` public surface grew: {hidden} items, ceiling is \
         {MAX_HIDDEN_FAMILY_ITEMS}.\n\n\
         A hidden item is one a crate of the engine family reaches in another; \
         anything else stays restricted. Name the crate that reaches it before \
         raising the ceiling."
    );
    assert!(
        hidden + STALE_CEILING_SLACK >= MAX_HIDDEN_FAMILY_ITEMS,
        "the hidden surface is {hidden} items but the ceiling is still \
         {MAX_HIDDEN_FAMILY_ITEMS}. Lower MAX_HIDDEN_FAMILY_ITEMS in \
         tests/public_surface.rs to {hidden}."
    );
}

#[test]
fn hidden_items_count_apart_from_the_documented_surface() {
    let source = "\
pub fn documented() {}
#[doc(hidden)]
pub fn reached_from_another_crate() {}
#[doc(hidden)]
#[inline]
pub const fn hidden_under_more_attributes() {}
/// Documented, then hidden.
#[doc(hidden)]
pub struct Hidden;
#[doc(hidden)]
pub(crate) fn restricted() {}
#[doc(hidden)]

pub fn separated_by_a_blank_line() {}
";
    assert_eq!(count_source(source), (2, 3));
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
fn a_moved_module_reexported_whole_counts_as_its_declaration_did() {
    assert!(!is_public_item("pub use rspice_core_base::config;"));
    assert!(!is_public_item(
        "pub use rspice_core_base::family_features;"
    ));
    assert!(!is_public_item("pub use rspice_core_devices::device::*;"));
    // Anything narrower or wider than one whole module still counts.
    assert!(is_public_item(
        "pub use rspice_core_base::config::SpiceDialect;"
    ));
    assert!(is_public_item(
        "pub use rspice_core_devices::device::mosfet::*;"
    ));
    assert!(is_public_item(
        "pub use rspice_core_base::{config, solver};"
    ));
    assert!(is_public_item("pub use rspice_core_base::Value;"));
    assert!(is_public_item("pub use rspice_veriloga::ir;"));
}

#[test]
fn every_family_module_reexport_names_a_module_of_that_crate() {
    let roots = family_src_roots();
    for root in &roots {
        for path in rust_sources(root) {
            let source = fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
            for line in source.lines().map(str::trim_start) {
                let Some((crate_dir, module)) = family_module_reexport(line) else {
                    continue;
                };
                let src = roots
                    .iter()
                    .find(|root| {
                        root.parent()
                            .and_then(Path::file_name)
                            .and_then(|name| name.to_str())
                            == Some(crate_dir.as_str())
                    })
                    .unwrap_or_else(|| {
                        panic!(
                            "{}: `{line}` names {crate_dir}, which is not a crate of the family",
                            family_relative(&path)
                        )
                    });
                let lib = fs::read_to_string(src.join("lib.rs"))
                    .unwrap_or_else(|error| panic!("read {}: {error}", src.display()));
                assert!(
                    lib.lines()
                        .any(|declared| declared.trim() == format!("pub mod {module};")),
                    "{}: `{line}` is counted as the declaration of a module, but {crate_dir} \
                     declares no `pub mod {module};`",
                    family_relative(&path)
                );
            }
        }
    }
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
    let solver_source = read_family_src("analysis/harmonic_balance/solver.rs");
    let device_source = read_family_src("analysis/harmonic_balance/solver/devices.rs");
    let api_source = read_family_src("analysis/harmonic_balance/solver/nonlinear_api.rs");
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
