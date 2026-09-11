# Unified mixed-simulation execution record

This record executes [the approved plan](implementation-plan.md). The full
MS00–MS15 scope remains required. Implemented behavior and release qualification
are tracked separately; the initial bug-fix milestone is not vendor parity.

## Integration and ownership

- Baseline fixes: `e89b1494b` ([evidence](fixes.md)).
- Integrated remote baseline: `c235715a0`, retaining the simultaneous flow-probe
  equations and the core sensitivity/netlist fixes through `0552bb87b`.
- Integration worktree: `codex/unified-mixed-fixes-20260910`. Publish coherent
  commits by normal fast-forward pushes to remote `main`; fetch and preserve
  concurrent commits before retrying a rejected push. Never force-push.
- This work owns mixed circuit integration. Existing compiler/runtime and
  general core workstreams retain their packages; reconcile their published
  changes at each integration boundary. No additional agents were started.

## Reference contract and open qualification dependencies

The normative language target is Accellera VAMS-2023, with separately recorded
SPICE dialect behavior and vendor extensions. Compare defined, race-free
behavior; do not infer a scheduling requirement from an intentionally racy
example. Scope includes every mixed capability in the approved plan, including
applicable advanced analyses, RNM, authored connections, and physical tablets.

A licensed Spectre AMS Designer reference installation is not currently available,
as confirmed by the user. Vendor accuracy/performance comparison is therefore
**unqualified**, while implementation continues against the standard and
independent circuit expectations.
Record release, solver/digital engine versions, licenses, model revisions,
options, tolerances, hardware/thread count and cache state before reference runs.
Physical tablet and shipping-browser execution also require actual target
evidence; native execution is not a substitute.

## Shared implementation contracts

1. **Source authority.** Compilation and connection selection consume the same
   active preprocessed closure and discipline definitions. A sealed virtual key
   never authorizes ambient filesystem discovery. Include/configuration changes
   invalidate dependent artifacts. Repeated selected modules from one closure
   register its specification once. Distinct specifications require explicit
   configuration selection; include traversal order is not a selection policy.
2. **Time authority.** Elaboration resolves each module's units and precision.
   Queue ordering retains checked exact discrete timestamps and scheduling
   regions. Analog physical activation/root time and digital reporting time
   remain distinct. An immediate causal response cannot consume an unrelated
   future timer. Cross-module rescaling and delayed assignment semantics belong
   to this contract, not individual host adapters.
3. **Graph authority.** Stable hierarchical instance/port/net/driver identities
   precede solver allocation. Conservative nets retain discipline/nature data;
   logic nets retain shape, signedness and four-state resolution; real nets
   retain their resolution policy. Deliberate event connections join directly.
   Existing electrical deck nodes and their physical loads keep their meaning.
4. **State authority.** Accepted state is circuit-wide. Prepare a trial from it;
   settle analog candidates, eligible discrete regions and cross-domain events;
   validate all participants; then publish one acceptance. Rejection restores
   queues, frames, drivers, analog history, converter state, random state and
   effect positions. Observations do not execute stateful analog code again.
5. **Binding authority.** Each cross-domain access has an owning declaration,
   type, dependency set and activation contract. Analog values/flows come from
   the appropriate solved candidate; discrete writes invalidate dependent
   analog work. Illegal writers are diagnosed at their source.
6. **Persistence/capability authority.** Runtime, cache and checkpoint schemas
   distinguish changed semantics. Checkpoints store stable semantic state, not
   executable addresses. Analyses, APIs and UI use the same required-capability
   inventory. Refusing a required valid case leaves its row open.

Concrete Rust graph/time/coordinator types will be introduced in MS02–MS05 at
their owning runtime layer and used by compiler and product adapters. The
contracts above govern their design and integration, including rollback and
checkpoint representation, before optimizing partitions or caches.

## Requirement and qualification inventory

| Required behavior | Packages | Implementation at this checkpoint | Qualification |
|---|---|---|---|
| Scheduled/startup analog sampling and unrelated timer isolation | MS02/03/05/06 | Reproduced baseline defects fixed; full contract open | Existing 10 mixed regressions pass after integration |
| Active source, macro/include, cache and virtual connection closure | MS01 | Artifact transport, shared file preparation, cross-root dependency consistency and named global rule selection implemented; full configuration binding remains | Focused file/virtual, source-refresh and selection cases pass |
| Complete resolved module timing and scheduling regions | MS02 | Declared module scales, constant delays, declaration-owned time queries and scaled analog compatibility time implemented; remaining timing semantics open | Focused compiler and mixed circuit cases pass; whole-circuit qualification pending |
| Circuit-wide analog/digital handshake, roots and LTE/retry | MS03 | Host-level baseline; design-wide work open | Pending |
| Typed SPICE/HDL/XSPICE graph, hierarchy and effective parameters | MS04 | Scalar specialization baseline; graph open | Pending |
| Global scheduling, resolution, atomic acceptance and effects | MS05 | Open | Pending |
| Legal values/events both directions, shapes and widths | MS06 | Scalar/real input baseline; full contract open | Pending |
| Authored connect bodies and rule/configuration selection | MS01/07 | Bodies must still be elaborated/executed | Pending |
| RNM, bidirectional resolution, ties, loading and timed conversion | MS04/07 | Open beyond basic one-way bridges | Pending |
| Coupled OP/DC/AC/noise and dependent valid analyses | MS08 | Open beyond transient startup | Pending |
| Periodic/RF/hybrid state and applicable advanced analyses | MS09 | Open | Pending |
| Persisted full mixed accepted state and continuation | MS10 | Open | Pending |
| Portable, x64, AArch64, Wasm and generated execution | MS11 | Partial; digital backend completion open | Pending required execution rows |
| EGUI schematic/deck, CLI, Python, Wasm and debugging workflows | MS12 | Partial route support | Pending integrated workflows |
| Shipping desktop/browser/physical tablet lifecycle and capacity | MS13 | Open | Actual target qualification pending |
| Throughput, cold/cached setup, memory, scaling and cancellation | MS14 | Open | Reference installation/budgets pending |
| Complete independent integrated release evidence | MS00/15 | Open | Not production-qualified |

The plan's UF01–UF22 table supplies finding-level traceability. Each delivered
increment adds its specific commit and decisive evidence below. These rows may
be split into finer cases as language/analysis inventories are implemented;
splitting must preserve every parent requirement.

## Verification budgets and evidence

Run only the directly affected cases during implementation. Reserve the broad
backend/device/analysis/deployment matrix and release checks for their integrated
milestones. A failed compile is fixed and the same target retried; it does not
justify unrelated suites.

For connection-closure fixtures, defined logic values must match exactly and
the unloaded 1 V conversion must agree within 1e-9 V. Source identity and
serialization integrity must match exactly. Existing timing regressions retain
their original thresholds and are reused, not loosened. Additional analog/event
budgets are specified per new reference circuit before observing comparison
results. Performance budgets require the reference workload/hardware contract;
invented throughput numbers do not qualify parity.

| Increment | Evidence | Outcome |
|---|---|---|
| Remote integration and distinct schema/cache versions | `cargo test --locked -p rspice-core --no-default-features --features veriloga --test veriloga_mixed_signal_regressions --target-dir target/unified-mixed-fixes --jobs 2` with test debuginfo disabled | 10 passed; no full suite run |
| Active connection closure transport | Core targets `veriloga_connect_source_closure` and `veriloga_mixed_signal_regressions`, filtered to `connect`, using the same portable build options | 2 passed: included file rules and serialized virtual models with shared rules/cache reuse |
| Connection integrity and parameter-source reuse | Compiler target `mixed_runtime_compile`, filtered to `connection_closure_is_retained_validated_and_reused_for_specialization`, no default features | Passed: retained user definitions, specialization, round-trip, missing/lost/altered payload rejection |
| Immutable preparation (`0b92805e8`) | Compiler target `prepared_source`, no default features | 2 passed: multiple modules compile with a removed include, captured identities remain correct, a new preparation sees changed bytes, source limits apply, standalone libraries need no device module |
| Engine source groups and cache provenance | Core target `veriloga_connect_source_closure`, portable Verilog feature | Both cases pass: shared virtual rules/cache reuse and a source edit during multi-module compilation followed by a fresh run |
| Cross-root source consistency | Core target `veriloga_connect_source_closure`, filtered to `snapshot`, portable Verilog feature | 2 passed: same-root frozen compilation and cross-root conflict/retry |
| Named connection configurations | Compiler target `prepared_source`, filtered to `named_connection`, no default features | Passed: independent 1 V/5 V alternatives, empty block, exact-case selection, duplicate module/block rejection |
| Circuit configuration selection | Core target `veriloga_connect_source_closure`, filtered to `named_configuration`, portable Verilog feature | 2 passed: file/cached/serialized-virtual 1 V/5 V choices with parameter specialization; include-order independence, unknown/ambiguous names and explicit empty rules |
| Source-qualified configuration selection | Core target `veriloga_connect_source_closure`, filtered to `source_qualified`, portable Verilog feature | Passed: duplicate Shared configurations select 1 V/5 V by import alias across file and transported virtual roots; unknown and multiply bound aliases reject |
| Prepared virtual sources and standalone connection transport | Compiler targets `prepared_source` and `virtual_source`, filtered to `virtual`, no default features | 2 passed: file/virtual artifact equality, serialization/integrity rejection, one front-end execution across two module emissions, cancellation, existing exact nested dependency receipt |
| Standalone virtual library circuit registration | Core target `veriloga_connect_source_closure`, filtered to `standalone_virtual`, portable Verilog feature | Passed: a zero-device library selects 1 V/5 V conversion; cache/source budgets, conflicting device registration, no partial installation and wrong-kind module selection preserve valid state |
| Existing atomic device registration after typed-cache integration | Core library tests filtered to `engine::builder::veriloga_cache::tests::plural_`, portable Verilog feature | 3 passed: whole-batch install, aggregate-budget rollback and installed-key collision rollback |
| Product connection-source import, snapshot and worker transport | UI library tests with `browser-worker`, filtered to `standalone_connection`; only the worker case rerun after correcting its waveform-label assertion | 4 focused cases passed: HDL-only import/integrity, exact deck dependencies, two-source snapshot provenance, serialized worker execution with 1 V/5 V selected outputs |
| Project and signed-PDK mixed model adapters | UI library tests with `browser-worker`, filtered to `unified_mixed_`, test debuginfo disabled | 2 passed: explicit editor/configured artifact equality with macros/include paths; module/binding/backend enforcement; authenticated PDK install and transport; both circuits switch from 1/3 V to 1/2 V within 1e-9 V |
| Declared module timing and constant delays | Compiler `module_timing` integration target, no default features; only the failing delay-expression case rerun after the parser correction | 2 passed: active include scope, inactive directives, reset, mixed module precisions, rounding, wide integer arithmetic, overflow refusal, canonical transport and timing identity |
| Physical mixed timing from files and sealed sources | Core `veriloga_module_timing`, portable Verilog feature | Passed: 451 ps and 1.3 ns breakpoints within 1e-20 s; SPICE-loaded conductance produces 1/3, 1/2, 1/3 V within 1e-9 V through both entry points |
| Existing native digital timing entry points | Core library `timescale` filter, portable Verilog feature | 5 existing cases passed: 2 digital-host cases and 3 VCD/projection cases selected by that filter; no full suite |
| Shipped artifact refresh for canonical schema 37 | Authoritative `rspice-veriloga-gen regenerate-builtins --jobs 2`, generator profile, isolated target directory | All 43 models regenerated; analog output bytes remain unchanged, generator manifest refreshed |
| Declaration-owned time queries | Compiler `module_timing` and `canonical_device` targets, `module_time_queries` filter, no default features | Portable device and executable generated Rust agree on the shared-terminal conductance at three physical times within 1e-12 S; seconds-sized query retains real division; malformed arity, missing fallback names and string fallback refuse |
| Time queries in digital defaults, delays and analog coupling | Core `veriloga_module_timing`, `module_time_queries` filter, portable Verilog feature | Passed: parent/child declarations resolve separately; query-based delay switches a loaded circuit at 0.5 ns within 1e-20 s and produces 1/4 V then 1/3 V within 1e-9 V |
| Shipped artifact refresh for canonical schema 38 | Authoritative `rspice-veriloga-gen regenerate-builtins --jobs 2`, generator profile | All 43 models regenerated; analog output bytes unchanged; generator digest `d53ce225d4e027cfb790448eb459e16aeed9f01b366bbab53eec844784b104a3` |

The source-transport increment advances canonical schema to 33. The subsequent
prepared-source integration advances core disk cache format to 70, so records
whose dependency fingerprints were obtained by a post-compilation reread are
rebuilt. Connection source is stored once
when it can also supply mixed parameter specialization. Sealed paths bypass
filesystem rule discovery. The engine no longer runs a preliminary textual
connection scan: it resolves cached models first, then prepares each needed file
group once for its module selections and connection specification. Only one
analyzed tree is retained at a time; runtime artifacts use the existing bounded
cache. Source expansion receives the engine's byte/depth limits. Browser cache
misses return a registration diagnostic without filesystem preparation.

The source-edit circuit has two 1 mS model conductances behind a 1 kohm resistor,
so its 1 V input gives 1/3 V. Editing the shared include during the first module's
emission leaves both modules at 1 mS in that run. The next run recompiles both at
2 mS and gives 1/5 V, proving that cached old behavior is not labeled with the
new include's identity. Standalone connection rules keep both logic outputs at
1 V. All voltage budgets remain 1e-9 V.

Different file roots also share a dependency-version ledger during elaboration.
If the same dependency was consumed with different bytes, the build identifies
the dependency and both roots and requests a stable source snapshot. A retry
can reuse or invalidate the captured cache entries normally. Full
configuration binding remains MS01 work; authored body execution remains MS07.
Generated artifact regeneration and the complete target matrix
remain integration/release work after the shared schemas stabilize.

The compiler now retains each named connection configuration and its declaration
span. Selecting a block excludes every alternative's insertion and resolution
statements. Duplicate declarations cannot silently overwrite a module.
`.options connectrules=NAME` now chooses one case-sensitive block from all active
source closures before mixed instances are allocated. The default is the sole
declared block; alternative blocks require explicit selection. Reusing a closure
through several device artifacts does not duplicate its configuration. Unknown
or ambiguous choices report available names, roots and preprocessed declaration
offsets. Empty selected blocks do not silently restore automatic default rules.
The named option participates in the existing semantic netlist fingerprint;
unchanged model code remains cacheable because selection happens at elaboration.
The two new circuit cases retain the 1e-9 V budget. No broad suite was run.

`.options connectrules_source=ALIAS` now restricts selection to one explicitly
aliased `.VERILOGA` import. The SPICE alias is case-insensitive; the rule name
remains case-sensitive. Selection uses the import's existing resolved source
identity and remains stable when a file path becomes a sealed virtual key.
Content deduplication occurs after source filtering, so an earlier identical
unselected closure cannot hide the requested import. Both options participate
in the netlist configuration fingerprint.

Remaining MS01 items include hierarchical library/view
binding and explicit propagation of connection-only artifacts
through all product adapters. The current named selector is design-wide and
does not claim those broader configuration capabilities.

Virtual discovery and compilation now share `PreparedVirtualSource`, which also
accepts a standalone connection library. Repeated module emission reuses the
analyzed tree and preserves the source map, active dependency graph, limits,
compiler options and runtime contract identities. Source preparation and module
emission expose cooperative cancellation without replaying completed phase
callbacks. A separate version-1 `ConnectionLibraryArtifact` carries connection
source without manufacturing a device ABI; its framed BLAKE3 identity covers the
schema, logical source package and exact preprocessed source. File and virtual
preparation produce identical artifacts for the same logical source. Product
transport of this standalone artifact remains integration work.
The concurrent hierarchy/flow-probe fix `45cfe35ce` was preserved during rebase;
it advances canonical schema to 34 and core disk cache format to 71 and updates
the generated model catalog. The two directly affected virtual-source cases
passed again after that compiler integration. Broader release qualification
remains pending.

Core source registration now accepts device and standalone connection artifacts
in one atomic batch. A typed source entry replaces the device-only cache value;
both kinds use the existing bounded cache, common collision checks and one commit
point. Connection source is validated and bounded before retention. The builder
resolves its registered rules without compiling or inventing a device module.
Failed source/cache budgets and a device/library key collision leave the prior
entries usable and the pending candidate absent. A standalone library cannot be
selected as a device. This changes in-memory registration, not the disk device
record schema. Browser workers and product import flows still need to carry the
new source entry type end to end before those deployment routes are qualified.

The product source set now retains standalone connection libraries alongside
device runtimes. Combined sets preserve both inventories through generated and
manual decks, deferred-source validation, snapshot identities, worker transfer
and the existing atomic core registration. Library import aliases remain intact;
unaliased libraries receive a deterministic reserved alias. Source keys and
aliases are checked across both kinds. The library's outer SHA-256 identity
covers its source key, closure digest, alias and validated compiler artifact
identity. Missing or altered inventories are rejected. Device JIT preparation
continues to enumerate only executable device entries.
Bundle import uses source preparation, so a zero-device connection file is no
longer rejected by module discovery. Native and retained imports recognize
authenticated HDL dependency edges as library contents without requiring a
dummy SPICE `.model` or `.subckt`. The focused fixture contains only HDL import
directives and their source files.

Model-library compilation prepares each logical root once for all selected
modules and opts into mixed runtime reports, matching the core's paired
analog/digital host contract. Project-editor and signed-PDK compilation still
use analog-only defaults and need explicit capability integration; this increment
does not claim those routes are complete. Worker request protocol advances to
10, with a required connection inventory (including explicit empty lists).
The browser transport fixtures were updated to that request shape and to the
already-current response protocol 20. Actual browser execution remains pending.
The four focused native product cases passed. The worker case uses the actual
native worker execution and result transport with a mixed HDL driver and a
standalone connection library; its 1 V/5 V expectations retain the 1e-9 V budget.
The initial worker assertion was corrected to use the bridge's existing `q`
voltage label, then only that case was rerun. No broad suite or actual browser
execution was run for this increment.
The concurrent changes through `d1e3e6c4e` were integrated before executing
those tests. That compiler fix advances canonical schema to 35 and core disk
cache format to 72. The earlier UI dependency build was interrupted before
tests ran; the combined build reuses those dependencies. The statistical
parameter and extreme-scale derivative fixes are also retained.
The subsequent integration retains `5834ad65d`'s statistical-parameter capture
fix. Its changed paths inspect statistics and user functions; the focused mixed
decks use neither, so those cases were not repeated for that rebase. The full
combined regression matrix remains an integration/release gate.

Project-editor, configured project-cell and signed-PDK model routes now retain
the canonical digital plan through shared mixed compiler options. Signed-PDK
admission uses those options as well as execution, while retaining signed source
digests, dependency checks and terminal contracts. The project build profile
lives in the shared simulation layer: configured execution now consumes its
macros, include paths, exact entry-module restrictions, cell-binding checks and
backend qualification requirements instead of using compiler defaults. The
explicit-module editor and configured routes produce identical sealed runtime
artifacts in the focused case. The legacy implicit single-file editor route
still uses its in-memory compilation receipt; broader receipt unification remains
open.

The two product cases execute a digital `reg` changing an analog conductance
behind a SPICE resistor. Their 1/3 V and 1/2 V expectations use the predeclared
1e-9 V budget. The first attempt exposed the compiler's existing refusal of a
module-level `integer` assigned by a digital process: digital lowering reports
it as an analog variable. The adapter fixtures now use an explicitly supported
`reg`; the integer ownership/lowering case remains required MS06 work, not a
qualified language form. Both cases then passed. Required generated-Rust
qualification continues to reject digital process execution; retaining an
analog runtime does not certify a missing digital backend. No broad suite or
actual browser/tablet run was performed.

The module-delay timing increment introduces validated `ModuleTimeScale` and
`DigitalTiming` contracts. Active preprocessor directives are retained at their
stream positions; the parser captures the scale at each module declaration.
HDL elaboration preserves each child's scale instead of adopting its parent's.
Each process records its owning module timing, and the plan stores the finest
elaborated precision. Constant delays round at module precision before exact
integer conversion to plan ticks; whole integer delay arithmetic avoids a
floating-point round trip. The portable process request now names plan ticks.
Both the digital convenience host and mixed transient host use that precision;
the convenience API exposes it for stimulus authors. Precision values through
100 s are representable, while the existing physical tick-range guard remains.
Missing or altered timing metadata cannot silently reuse a sealed digital plan.
Canonical schema is 37 and core disk cache format is 74.

The source includes a 10 ns / 100 ps child and a 1 ns / 1 ps parent. The child's
0.125-unit delay rounds to 1.3 ns; the parent's 0.4505-unit delay rounds to 451 ps.
Their discrete state changes one analog conductance in the existing shared
SPICE equation system. File and transported execution meet the declared timing
and voltage budgets. The initial circuit fixture was corrected to use the
existing case-insensitive SPICE node lookup and to omit module overrides on
sealed sources, whose module selection is already fixed. The parser check also
exposed and fixed a real defect: it previously rejected every parenthesized
delay expression as a min:typ:max tuple. Only actual tuples retain that refusal.
The first generator attempt detected source formatting during generation and
published nothing; regeneration then succeeded against stable source inputs.
The final refresh includes the new timing module in the generator's explicit
source-digest inventory, so later timing changes invalidate shipped artifacts.

This is still partial MS02. Complete dynamic/continuous and nonblocking delayed
assignment semantics, min:typ:max corner selection, time values and reporting,
and applicable SystemVerilog timing declarations remain open. Inspection found
that analog `$realtime` still lowers as unscaled `$abstime` in all analog paths;
`$simparam("timeUnit")` and `timePrecision` also need module-specific values.
Do not qualify those queries from the delay tests. Resolve them at the owning
module before analog hierarchy flattening so every backend agrees. Global
precision/causality across separate SPICE/HDL/XSPICE participants remains MS04/05
work, and actual browser/tablet qualification remains pending.

The following increment resolves `$simparam("timeUnit")` and `timePrecision`
before hierarchy flattening. Both values retain real-number typing, even for
1 s, 10 s and 100 s units. Parameter defaults, constant delay expressions,
digital arithmetic and analog equations use the owning module's declaration.
Host simulator-parameter overrides no longer substitute for these declarations.
Known timing queries discard unused fallbacks before function materialization;
numeric domain errors and output-argument writes in those fallbacks do not run.
Discarding them retains declared-name checks, registered/user function arity
checks and the numeric fallback requirement. Digital fallbacks retain the
existing digital-expression checks.

Analog `$realtime` now lowers to seconds-valued `$abstime` divided by the owning
module's time unit. This is an explicit retained compatibility behavior: the
[VAMS-2023 LRM](https://www.accellera.org/images/downloads/standards/v-ams/VAMS-LRM-2023.pdf),
section 9.10, deprecates analog `$realtime` and defines `$abstime` in seconds.
Digital clock-query execution is still open. Canonical schema advances to 38
and the core disk cache format to 75 so artifacts using the old alias/query
semantics cannot be reused.

The first new backend assertions incorrectly summed opposite branch reference
directions after hierarchy flattening. They were corrected to compare the
assembled shared-terminal Jacobian and KCL contribution, retaining the 1e-12
budget. The negative fixture then exposed and fixed an undeclared fallback name
being hidden by early query resolution; only the failing portable case was
rerun after that correction. The three focused cases pass. No full suite,
actual browser, physical tablet or vendor-reference run was performed.
The refresh retains the same source-tree digest and updates the authoritative
generator manifest. The concurrent `00aca9e51` finite-tanh derivative fix touches
native SPICE behavioral expressions and is preserved at integration; none of
these timing fixtures uses tanh. Combined release verification remains pending.

## Next implementation work

Carry standalone library entries through project-editor and signed-PDK bindings;
those adapters still require an executable module selection. Keep their compile
profiles, terminal contracts and backend qualification consistent with the mixed
model routes. Qualify the actual
browser-worker path at its integrated target milestone.
Continue hierarchical library/view binding with the typed design graph.
Continue MS02 with digital clock queries and remaining delay semantics,
then MS04 typed graph and the MS05 coordinator. The first circuit-wide slice must
include two HDL instances, an XSPICE participant, a loaded SPICE boundary,
off-grid timing and a rejected trial. Continue through every remaining milestone
in the approved plan; no parity claim is made at this intermediate checkpoint.
