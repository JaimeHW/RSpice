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
| Complete resolved module timing and scheduling regions | MS02 | Declared scales, runtime waits, delay/event-controlled NBA delivery, computed event expressions, declaration queries, digital activation clocks and scaled analog compatibility time implemented; remaining timing semantics open | Focused compiler and mixed circuit cases pass; whole-circuit qualification pending |
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
| Digital activation clock expressions and wide arithmetic | Compiler `digital_process_execution`, `digital_clock_queries` filter, no default features | 2 passed after integration: half-unit rounding, suspension, exact integers beyond f64, 32-bit wrap before widening, signed 129-bit arithmetic/comparisons, 257-bit shift counts, wide decimal/unsized literals, X/Z and zero division, missing clock/invalid type/arity rejection |
| Literal admission after exact wide lowering | Compiler library `lexer::tests::malformed_based_literals_fail_closed`, no default features | Passed: malformed and over-budget forms still refuse; valid wide based forms retain bit-plane tokens |
| Digital clocks on the event and mixed hosts | Core library `digital_clock_queries` filter, portable Verilog feature | 2 passed after integration: hierarchical 10 ns/1 ns units at 1.5 ns; a 0.65 ns physical activation reports tick 1, preserves #0, isolates a pending tick-1 timer, rolls back/retries and restores an in-memory checkpoint; 1e-12 A and 1e-20 s budgets |
| Shipped artifact refresh for canonical schema 40 | Authoritative `rspice-veriloga-gen regenerate-builtins --jobs 2`, generator profile | All 43 models regenerated; source bytes unchanged relative to the preserved physical-branch fix; generator digest `c68a11fbaf28dd50961182701765bffe39801b5fd94d250ea757e1b00968f152` |

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

Digital time reporting now has a typed `DigitalTime` CFG node and a required
`DigitalClock` environment contract. Reads stay in their process blocks across
suspension. The plan supplies design precision and the process supplies its
module unit. `$time` rounds by integer quotient/remainder; `$stime` takes its
low 32 bits before the enclosing expression widens it. `$realtime` scales the
reporting tick; `$abstime` keeps physical seconds. The host publishes a fresh
clock for every activation, retaining physical analog time through its immediate
and inactive-region consequences. Accepted-state clones and the existing
in-memory checkpoint restore these records. Persisted mixed restart remains open.
The definitions follow IEEE 1364-2005 section 17.7 and VAMS-2023 section 9.10;
the physical/reporting split retains the section 7.3.6.1 causal contract.
[IEEE standard text](https://www.eg.bucknell.edu/~csci320/2016-fall/wp-content/uploads/2015/08/verilog-std-1364-2005.pdf),
[VAMS-2023](https://www.accellera.org/images/downloads/standards/v-ams/VAMS-LRM-2023.pdf).

The widened `$time` fixture exposed pre-existing arithmetic/comparison behavior
that mapped every operand above 64 bits to unknown. Known wide operands now use
`num-bigint` 0.4.8, already present in the repository dependency graph; the
machine-word fast path remains. Results wrap at declared width, signed division
truncates toward zero, and remainder follows the dividend. Wide known shift
counts remain known. Valid wide based literals now use the digital bit planes,
unsized values retain their digits, and decimal magnitudes beyond 128 bits use
the existing literal budget. The analog integer decoder retains its separate
representability restriction. Full cross-domain coercions remain MS06 work.

The first test link failed because C: was full, before any test could execute.
Cargo's package-scoped cleanup removed 5.9 GiB of this worktree's disposable UI
build artifacts; source files and other worktrees were preserved. The failed
link was retried. Subsequent retries addressed the observed wide-arithmetic and
literal failures. Concurrent `ee4ae21b3` is preserved, including physical-branch
mapping and generated model changes. The combined artifact schema is 40 and
disk-cache format is 77, distinct from that commit's schema 39/cache 76. The five
focused cases pass on the combined source. No full suite or actual browser,
tablet or vendor-reference execution was performed for this increment.

Procedural waits now evaluate a typed `DigitalDelayTicks` node at encounter.
Constants use the same digital width/signedness rules as local variables, signal
reads, real expressions and time queries. Conversion rounds at module precision
and then rescales to design ticks. IEEE 1364-2005 sections 9.7.1 and 9.7.5 require
X/Z delays to become zero and delay-input reads to participate in implicit
sensitivity. Negative values convert to unsigned 64-bit time; the interpreter's
signed-tick capacity and the host's exact physical-time capacity remain explicit
range checks. Dead code containing a finite, out-of-range constant delay no
longer fails compilation; executing that wait reports a numeric delay error.
[IEEE standard](https://www-inst.eecs.berkeley.edu/~eecs151/sp20/files/verilog/verilog-std-1364-2005.pdf).

The focused run exposed an unsized decimal sizing defect that the former
constant-delay shortcut hid. Such literals now retain sufficient signed width,
with a 32-bit minimum. Sized arithmetic still wraps at its expression width.
The same run encountered two existing MS06 limitations: module-level real
variables without discrete ownership are not digital signals, and a known
64-bit based literal such as `64'd9007199254740993` is rejected by scalar parsing.
The runtime input fixtures use declared real nets and exact integer expressions;
those broader ownership/literal gaps remain required work, not qualified support.

Evidence for this increment: six compiler process delay cases, two module-timing
cases, and one mixed-host case pass. Failed cases alone were retried after the
observed sizing, fixture-ownership and fixture-resolver corrections. The mixed
case covers an off-grid analog activation, unknown zero delay, a rounded real
delay computed from `$realtime`, independent timer isolation, analog stamping,
rejection/retry and accepted in-memory checkpoint restoration. The small IR
shape target is native-feature gated and selected zero tests under the portable
command; it is not counted as executed evidence. No full suite or actual
browser/tablet/reference run was performed. Canonical schema is 41; cache format
is 78. The authoritative generator regenerated all 43 shipped models; only the
generator manifest digest changed. Concurrent core commit `97e921d4f` is retained
at integration. It changes complex parameter directions and definition-time
bindings; these timing fixtures do not exercise those paths. Combined release
verification remains pending.

Delayed nonblocking assignments still require independent captured updates:
`q <= #d rhs` currently suspends the process through the common wait lowering.
This known semantic bug is the next timing implementation slice. Continuous
inertial/transport behavior, complete time declarations, persisted restart,
circuit-wide coordination and all later plan milestones remain open.

Delay-controlled nonblocking writes now carry an optional converted delay operand
in canonical IR and a captured tick count in the runtime update contract. The
lowerer emits the write at encounter and continues in the same block; blocking
intra-assignment waits retain their existing captured-RHS suspension. `#0` and
X/Z delays enter the current NBA region without an inactive-region suspension.
The ordinary digital expression evaluator supplies widths, signedness, module
rounding and range checks before capture.

The host stores future payloads by due tick, with one permanent scheduler target
and one kernel wakeup per due tick. It does not allocate a process or a driver
identity per write. Timed requests are collected separately from already-ready
updates, avoiding scans of the entire NBA queue after every process activation.
When the event kernel reaches a due tick, the payloads enter the existing NBA
region queue. Active/inactive work still finishes first. Captures are applied in
order, retain partial-write behavior and survive source-process completion.
Deliveries count toward the scheduler's event budget. Existing accepted-state
clones/checkpoints include the future payloads; persisted restart remains open.

Four focused cases pass: the compiler's capture/continuation case, its existing
blocking intra-assignment case, an end-to-end digital ordering/range case, and a
mixed rejection/checkpoint case. Evidence covers runtime delay/RHS capture,
real and concatenated partial writes, current-slot zero/X delay, independently
dated writes without cancellation, reads before NBA promotion, source-process
completion, and explicit host-range refusal. The mixed case retains an unrelated
tick-1 NBA during a 0.65 ns analog activation; its captured real value is delivered
at tick 2, retained through rejection/retry, and replayed from an accepted
in-memory checkpoint. Current and breakpoint budgets remain 1e-12 A and 1e-20 s.
The initial mixed fixture used the reserved keyword `timer` as an identifier;
only that case was retried after correction. The two host cases were rerun after
separating timed requests from the ready queue to remove repeated scans. No full
suite, actual browser/tablet run or vendor-reference execution was performed.
Canonical schema is 42 and disk-cache format is 79. The authoritative generator
regenerated all 43 shipped models. Only the generator manifest digest changed;
the source-tree and emitted bundle digests are unchanged.

Event-controlled NBA captures (`q <= @(posedge clk) d`) remain a distinct required
fix: the current event-wait lowering still suspends the process. Implement their
independent subscriptions next, including changes before/after registration in
one activation, same-slot ordering, repeated captures and mixed rollback. Dynamic
LHS selections and deferred process-local storage are still MS06 requirements.
All other approved milestones remain in scope.


Event-controlled nonblocking assignments now carry an optional `DigitalWait`
in canonical IR and a captured `DigitalWaitRequest` at runtime. Direct signal
edges, real changes and implicit RHS sensitivity register independent one-shot
subscriptions without suspending the source process. Indexed subscriptions only
visit captures dependent on the changed signal. A monotonic, checked sequence
separates registration from earlier transitions in the same activation; ordinary
process event waits use that frontier too. Eligible captured writes retain source
statement order. Normal ready NBA traffic remains append-only, with sorting only
when event delivery appended an earlier capture after a later ready update.
Sequence exhaustion is an explicit run error rather than wrapping event order.

Four focused positive cases pass: compiler event capture/continuation, existing
compiler delayed capture/continuation, digital-host event order/one-shot delivery,
and mixed rejection/checkpoint replay. The host cases cover transitions before
and after registration, two captures delivered in source order despite reversed
trigger order, real captures, inactive-region reads, source-process completion,
and an ordinary event wait that must ignore an already-observed clock edge.
The mixed case captures physical time at 0.65 ns and delivers it on a later
1.65 ns analog activation, while an unrelated tick-2 delayed NBA remains pending.
Registration and delivery survive rejected trials and accepted in-memory replay.
Current and breakpoint budgets remain 1e-12 A and 1e-20 s. A stale internal
method argument caused a build failure and was corrected before host execution.
The negative compiler assertion initially expected only the lowering diagnostic;
the semantic stage can refuse the same unsupported term earlier. Its assertion
now accepts either source-located refusal, while still requiring compilation to
fail. Computed/selected event expressions and repeat controls remain required
MS02 work; rejecting them is not completion of those language requirements.

Canonical schema is 43 and disk-cache format is 80. With the drive nearly full,
Cargo's package-scoped cleanup removed 7.6 GiB of disposable compiler dev build
artifacts in this worktree. Other worktrees and the generator profile were
preserved. No full suite, actual browser/tablet run or vendor-reference execution
was performed. The user confirmed no licensed Spectre reference installation is
available; implementation continues and reference qualification stays pending.


The authoritative generator regenerated all 43 shipped models; only its manifest
identity changed (generator digest
`65a907b8de921d9347fc1dbb4eae6a580d16d2406c19a52cb2acf8b70b68e1f5`).
Concurrent `0f063c299` and `6e36744f3` are retained. Their AC source-amplitude
and parameter-sensitivity changes do not alter the event capture implementation;
combined full-release verification remains pending.

Computed event controls now carry ordinary typed expression roots in canonical
IR. Pure dependency programs are prepared once per process/root and shared
between activations. Each subscription owns its previous result; the store
indexes subscriptions by their input signals and observes them at each write,
before later writes can hide a short pulse. Both ordinary process waits and
independent NBA captures consume the resulting one-shot notifications. Accepted
state clones preserve these baselines, subscriptions and pending captures.
External/analog input publication and NBA promotion now establish the current
physical/reporting clock before evaluating dependent expressions.

The first focused compiler case exposed the existing constant-only bit-read
lowering. Runtime bit selection now has its own typed node. It retains declared
ascending/nonzero ranges, signedness, exact wide indices and unknown/out-of-range
X semantics. Static part selection and dynamic write targets retain their
separate contracts. Implicit sensitivity also follows system-function arguments.
Semantic analysis no longer rejects repeated input names or constant event
expressions; computed lists report no incomplete static signal-only metadata.
Event observation uses expression result changes and LSB edge classification,
per IEEE 1364-2005 sections 9.7.2 and 9.7.4; bit selection follows section 4.2.1.
[IEEE language reference](https://www.csie.nuk.edu.tw/~stpan/course/Verilog1964-2005.pdf).

Process-local and analog-owned event dependency bindings, indexed part selects,
repeat controls and dynamic write targets remain required work. Their present
refusal is not qualification of those valid language features. The circuit-wide
graph, scheduling/acceptance authority and all later approved milestones remain
open. There is no vendor-parity or production-readiness claim at this increment.


Five focused cases pass on the combined source: computed-event grammar, compiler
expression/dependency execution with artifact round-trip, exact runtime bit
indices, digital-host pulses/captures/clocks, and direct/computed mixed replay.
Evidence includes masked input changes, changes to an unselected bit, a changed
index, real expression value changes, repeated posedge/negedge terms, constant
expressions, 96-bit signed indices, unknown/out-of-range X, same-activation
pulses, source-order NBA delivery and late registration. The mixed fixture uses
`$abstime` inside both arm and ADC event expressions at physical 0.65 ns and
1.65 ns, with rejection/retry and accepted checkpoint replay. The independent
2 ns timer is not delivered early. Budgets remain 1e-12 A and 1e-20 s.

The first compiler failure identified the constant-only bit-read gap. Subsequent
build corrections resolved one missing enum arm, a host borrow lifetime and a
nonserializable semantic range type; the canonical range is now an explicit
integer tuple. Failed checks were retried after those corrections. The host
pair was rerun once after integrating concurrent `ce9503c7c` (analog switch
branches/accepted-mode discontinuities) and `cd8a0420d` (deferred resistors and
parameter directions). Combined schema 45 and cache format 82 are distinct
from the switch-branch commit's schema 44/cache 81. No full suite, actual
browser/tablet execution, performance qualification or vendor-reference run
was performed for this increment.


The final integration also preserves `60ddce38e` (direct/indirect parallel-branch
conflict validation) and `9e025c3dd` (finite complex parameter quotients). The
closing schema is 46 and cache format is 83, so artifacts from either concurrent
schema 45 implementation cannot be mistaken for the combined contract. The five
focused checks above ran before these last two commits; their changes concern
parallel analog contributions and complex parameter evaluation, which these
fixtures do not use. They were reviewed at integration, with final broad release
verification still pending. The generator identity is refreshed against the
final integrated compiler source.


The authoritative generator completed all 43 built-ins against the final source.
Only the generator manifest identity changed; the upstream switch-branch emitted
bundle is preserved. Final generator digest:
`80516a056315159b2ee0a0169a736b231a917158221eba7ec6c7673739d1ee7f`.
The second refresh was required by a concurrent compiler-source change arriving
during the first generation; it was not an additional simulation-test sweep.

## MS02 increment: counted event controls and exact repeat normalization

Intra-assignment `repeat (count) @(event)` now retains the evaluated count and
RHS for both blocking and nonblocking assignments. Zero counts branch around
all event-expression evaluation. Counts containing any X/Z and signed negative
counts normalize to zero; unsigned values keep their unsigned interpretation.
Real counts use the shared signed 32-bit integer conversion (nearest with half
away from zero) and reject nonfinite or out-of-range values before scheduling.
Integral counts retain their full width. Ordinary repeat loops now use this
normalization, fixing the previous partially unknown count and signed-negative
loop behavior. Generate substitution retains the repeat expression.

Runtime subscriptions count transitions without resuming the source process
on intermediate occurrences. Direct signal and real value-change waits,
computed event results and implicit RHS sensitivity share this contract.
Distinct matching transitions at one timestamp count separately. Computed
subscriptions refresh every dependent expression baseline after a write,
even if an earlier term already matched; stopping at the first matching term
would lose later negedges in a repeated posedge/negedge list. Count, RHS,
baselines and source ordering all belong to checkpointed pending state.

Four focused cases pass on the source integrated with `b60f26a4c` (scoped
numeric subcircuit parameters): compiler count normalization/zero bypass,
compiler capture/generate/artifact transport, host transition counting, and
mixed partial-count rollback/replay. Coverage includes signed and unsigned
negative expressions, partial X/Z, real rounding and invalid real counts,
96-bit subtraction across the 64-bit boundary, two generated repeat counts,
count/RHS mutation after capture, blocking versus NBA delivery, same-activation
input pulses and independent future timers. The mixed case checks direct and
computed subscriptions at physical 1.15 ns and 1.65 ns, rejects/retries the
intermediate and completing occurrences, and replays an accepted checkpoint
with one occurrence still pending. Current and time budgets remain 1e-12 A
and 1e-20 s.

The initial core build caught an import inserted inside documentation; that
edit was corrected before the passing host checks. An added invalid-count
fixture first exposed the existing ownership refusal for a module-level real
variable with no discrete owner. The fixture now uses a typed real input
(`wreal`) to test invalid runtime numeric conversion. General analog-owned
variable binding remains MS06 work; this increment does not close that gap.
Only affected focused checks were rerun after those corrections. No full
suite, browser/tablet execution, performance run or vendor-reference run was
performed. Schema 47 and cache format 84 distinguish this artifact contract.

Spectre reference qualification remains unavailable: the user confirmed that
no reference installation is available yet. Implementation of all remaining
MS00–MS15 requirements continues; these four cases do not establish commercial
production readiness or vendor parity.

The authoritative generator completed all 43 built-ins. Only the manifest
generator identity changed; the emitted analog model bundle is unchanged.
Generator digest: `9033856958cb8394330fcc8b75d26d1fe4599a0dab5055de7993efc873abeb6a`.

## MS05 increment: staged acceptance of owned HDL and XSPICE state

Mixed-host acceptance is now split into preparation and infallible promotion.
An exclusive reservation owns each validated trial; dropping it restores the
trial. The circuit validates runtime/generated analog models and prepares every
mixed host before promoting any of those states. A later host failure therefore
cannot leave an earlier host or an analog-only model committed. Initial,
ordinary and force-accepted engine paths use the joint HDL entry point.

The two transient acceptance tails now share an external-model acceptance
function. XSPICE candidate evaluation returns its error instead of logging it
and proceeding with acceptance. Copy-on-write instance/event snapshots retain
owned XSPICE state while analog and mixed candidates are evaluated and checked.
A failure restores those contexts, event values, pending events and the prior
error latch. Projected ideal-voltage outputs retain a compact undo list, so a
failed candidate also restores the solution entries the projection overwrote.
After the joint HDL barrier succeeds, XSPICE promotion has no fallible model
calls left. Xyce static-history capture occurs before promotion and is returned
only on success. Pure native-SPICE steps retain their external-model-free path.

Three focused cases pass. Two acceptance cases use two HDL instances, an
analog-only observer, two stateful XSPICE participants and native resistive
loads. They inject a failure in the later HDL or XSPICE participant, compare
owned XSPICE context images exactly, check earlier digital state and pending
2 ns timers, restore projected voltages, retry at physical 0.65 ns, and replay
a circuit checkpoint. An analog `$finish` candidate is absent after failure
and delivered once after successful acceptance. These cases exercise the
acceptance protocol with supplied candidates; they are not a proof of a full
multi-instance feedback solve. The existing deck-level
`rejected_timepoints_leave_the_digital_half_exactly_where_they_found_it`
regression also passes through the changed transient engine route.

The first new fixture omitted the engine's explicit digital-start operation;
that fixture was corrected before the passing checks. The two acceptance cases
were rerun once on the combined source after incorporating `5a7ccf963`,
`c937c7526` and `d5c03954b` (parameter derivatives, sensitivity refinement and
physical shunt options). The engine-route case also ran on that combined base.
The final fast-path guard excludes circuits with no external models. No broad
suite, platform execution, generator refresh or performance run was performed;
compiler/generated-model inputs and artifact schemas are unchanged.

MS05 remains open. This increment protects circuit-owned model/context state;
`CmContext` resources may contain shared external state. In particular,
`d_cosim` can advance an external runtime during `AcceptedStep`, and other
resource-backed models have their own probe/commit conventions. Their resource
transactions and deferred effects must join the barrier before a general
atomicity claim is valid. Native SPICE reactive histories and controller state
also remain in the parent stepper's acceptance sequence. The typed circuit
graph, one scheduling/precision authority, cross-instance event connections,
whole-circuit root/retry coordination and the complete MS00–MS15 qualification
remain required. Spectre reference execution is still unavailable.

The final rebase also includes `c3a3c36fc` and `a1ec45a3f`, which change
passive instance assignment replacement and expression evaluation during
netlist parsing. The checks above ran before those last parser-only commits;
the acceptance implementation is unchanged by that integration.

## MS05 increment: reversible external resources join acceptance

The acceptance barrier now journals registered shared XSPICE resources as well
as circuit-owned contexts and event state. A resource exposes an observational
capture and an exact restore contract. Checked access captures once per resource
registration across settle passes, before its first use; it also enlists resources
registered after the candidate snapshot. Ordinary resource access cannot bypass
transactional registrations. Existing memory/cache resources retain their
copy-on-write behavior.

On refusal, every captured resource is restored in reverse acquisition order,
including when another provider's restore fails or panics. Diagnostics retain
the original candidate failure and identify each failed instance/resource.
A failed restore poisons the resource registration. A separate permanent circuit
failure is shared with existing in-memory circuit clones, so reading the error,
restoring owned state, or losing a newly created registration cannot authorize
another simulation of uncertain external state. Freshly built circuits have a
fresh failure latch; pure native-SPICE circuits allocate no resource latch.

Reversible `d_cosim` registers its runtime through this contract and joins the
outer circuit transaction. Its probes still restore immediately, and standalone
calls retain their local undo image. Output processing now belongs inside the
same local transaction as initialize/startup/step; an invalid output-delay
parameter previously could fail after a successful external advance without
restoring that advance. Successful outer acceptance releases the images only
after the joint HDL and XSPICE promotion completes.

Eight focused checks passed before the final permanent-circuit-failure addition:
the two existing mixed/XSPICE acceptance cases extended with real shared-resource
state, three existing reversible-cosim contract cases, two new cosim cases for
outer rollback/commit and output-processing failure, and one journal case for
reverse restoration, provider failure/panic and invalidation across context
clones. The acceptance cases check resources created during the first candidate,
multiple accesses capturing once, a later HDL or XSPICE refusal, and successful
retry. The two final acceptance checks also passed after incorporating concurrent
`19d7b65c1`, `8faed906e` and `03c84e746` parser changes. The actual combined
baseline was `e445bfbea`, also including `3375c9d1d`, `db5f6d65d` and
`a6ac0f977` indirect-source/compiler updates and the guarded-source correction.
They additionally cover
failed restoration of a newly registered resource and repeated refusal through
an earlier circuit clone. Only these affected cases were rerun after the final
failure-latch change; all runs used the core library with the `veriloga` feature.

This remains an acceptance-protocol increment. The mixed fixture uses supplied
candidates and does not yet qualify the required whole-circuit feedback solve.
Irreversible `d_process`/cosim providers, deferred external effects, native-SPICE
reactive/controller acceptance, the typed graph and one scheduling authority
remain MS05 work. Successful circuit cloning is not an external-resource restart
image; persistent/replayable resource checkpoints remain MS10 work. No broad
suite, platform execution, generator refresh, performance qualification or
Spectre reference run was performed. Compiler/artifact schemas are unchanged.
The user has confirmed that no licensed reference installation is available;
commercial readiness and vendor parity remain unproven.

## MS05 increment: prepare thermal material state before model promotion

Thermal-resistor material evaluation now produces a compact candidate containing
only numeric material/load values. It does not copy retained expression scopes
or definitions. Both single-device advancement and the resistor-family entry
point prepare before mutation; a later resistor's invalid material can no longer
leave earlier resistors advanced. The family validates effective resistance and
conductance before committing any candidate and identifies the failing resistor.

The external-model barrier prepares thermal states from the final XSPICE-projected
solution before HDL promotion, then applies those states without another material
callback after the joint HDL barrier succeeds. A later HDL refusal discards the
thermal candidates. A thermal refusal takes the existing XSPICE context/resource
and projected-solution rollback path before any HDL state or accepted analog task
can be published. Ordinary and forced-acceptance tails use this ordering. The
native-only path uses the same transactional family advancement before controller
history updates; the old later advancement was removed from both tails.

Two focused cases passed on the `a236b2da9` baseline with the core library
and `veriloga` feature. One acceptance fixture combines two mixed
hosts, two XSPICE resources, an analog task observer and two thermal resistors.
It checks a later HDL failure, a later material failure in standalone family and
mixed acceptance, exact preservation of thermal/material/output values, resource
and voltage-projection undo, absence of accepted effects, and successful retry
using 0.75 V after projection rather than the supplied 1 V candidate. The other
case runs constant-power decks through the native-only and XSPICE acceptance
paths, checking every accepted temperature against the analytic energy balance
to detect skipped or duplicate updates.

This increment does not close native-history or controller atomicity. The
parent's reactive-history operation still rotates capacitors/inductors and
hysteresis before fallible behavioral-source and BJT acceptance, and MOS history
can depend on fallible parallel-worker preparation. Those operations need a
common prepared state before promotion; elapsed time, breakpoint consumption,
output retention and effect publication also require the full coordinator
contract. MS04 typed connectivity, circuit-wide event scheduling/root refinement,
the actual whole-circuit feedback slice and every remaining MS00–MS15 milestone
remain open. No broad suite or platform/vendor qualification was run.
The final rebase includes `4765859de` (controlled-source multiplicity derivatives
and parser plumbing). The two checks ran before that integration; its changes
were reviewed and do not modify thermal state or the acceptance implementation.
Compiler and generated-model inputs were not changed by this increment.

## MS05 increment: prepare fallible native history work before rotation

Native reactive acceptance now has explicit preparation and commit operations.
Preparation resolves BJT private state and terminal currents for the entire
family, evaluates every behavioral-source VM candidate, validates parallel MOS
history shapes and obtains the analysis worker pool. The commit operation uses
those prepared values and contains no fallible expression, BJT reconstruction or
worker-pool acquisition call. The existing transient entry point prepares before
rotating capacitor, inductor, hysteresis, transmission-line or junction history.

BJT candidates retain only newly evaluated charge/current/internal/predictor
values, rather than duplicating old history generations. Non-finite terminal
voltages and reconstructed history are refused with the device identity. The
standalone BJT acceptance operation used by periodic traversals also prepares
the full family before promotion. Behavioral candidates copy VM execution state;
compiled expressions, binding tables and parameter setup are not cloned. A later
current source cannot leave an earlier voltage source's SDT state accepted.
Standalone behavioral acceptance follows the same prepare/commit contract.

Two focused checks pass. A native acceptance case combines two BJTs, voltage and
current behavioral sources with SDT, a capacitor and an inductor. A later BJT
failure and then a later behavioral-source failure leave every BJT and passive
history unchanged. Retrying with zero input gives exactly zero integrated SDT
area; a rejected 10 V trial cannot contribute to accepted history. The fixture
also checks the standalone behavioral family entry point. The existing native
BJT lead-current regression passes its Trap/Gear and polarity cases with external
series resistances, preserving the numerical current/KCL checks.

An initial compile caught a misplaced worker-helper return during extraction;
that edit was corrected before executable checks. The first failure fixture put
NaN on the external base node behind a generated series resistor, so it did not
actually invalidate the BJT's bound terminal. The fixture now targets that actual
terminal. Only that corrected failure-path case was rerun on the combined
`e599fa164` baseline, including `dacde68df` and `df95d156b` passive-value/parser
changes. The existing BJT numerical regression passed on `86b69f439` before
those changes and the added terminal-finiteness check.

This separates native preparation from promotion but does not yet move native
promotion behind the HDL barrier. The next integration must carry the prepared
native token through XSPICE projection and joint HDL acceptance, invalidate
solution-derived snapshots when projection changes their inputs, then promote
all native histories without another fallible calculation. Native evaluated
cache rollback, controller time/breakpoint consumption, effects/output retention,
shared event connectivity and the actual whole-circuit feedback/rejection slice
still require the complete MS04/MS05 coordinator. All other MS00–MS15 requirements
remain in scope. No broad suite, platform or vendor qualification, performance run,
compiler-schema change or generated-model refresh was performed.

## MS05 loader correction: exclude mixed hosts from the Core-only DAE path

The specialized direct Xyce Level-2 Core loader explicitly supports a narrow
native device population. Its capability proof excluded runtime/generated
analog devices but omitted the separate mixed-host population, allowing a mixed
host's equations to be absent from that specialized load. It now checks the
common analog/mixed-family predicate. A circuit containing such a host therefore
uses the general loader that includes its equations.

The focused capability regression starts with an eligible native Core circuit,
attaches a mixed host on an existing node, and verifies the specialized route is
no longer eligible. It passed in the four-case joint-acceptance check on the
`db76aceff` baseline. This verifies dispatch eligibility, not numerical
qualification of every mixed Core feedback circuit.

## MS05 increment: one acceptance barrier for native, XSPICE and HDL history

Ordinary and forced transient acceptance now call the same model barrier with
borrowed native history storage. XSPICE evaluates and projects its candidate;
thermal material and native BJT/behavioral/parallel-MOS work prepare without
accepted-history promotion; every HDL participant validates before the native,
thermal and XSPICE histories advance. Reversible external-resource images are
released only after those promotions finish. The previous production tails that
committed native reactive state before HDL validation have been removed. The
standalone external/native wrappers now exist only as test adapters.

When XSPICE projection changes the solution, the barrier invalidates the supplied
VBIC, capacitor, MOS capacitance and gate-companion caches and refreshes native
trial bias. The accepted histories therefore use the projected solution rather
than cached quantities from the previous voltage. Xyce static F-B capture also
belongs inside this barrier for both native and mixed paths. Its direct Core Q/F
capture remains before the barrier on the narrowly qualified native-only path,
which now explicitly excludes all HDL hosts. Diagnostic timing labels this whole
phase as acceptance and counts it in ordinary and forced paths.

Four focused checks passed together on the `db76aceff` baseline. The new joint
case runs with two mixed hosts, an analog effect observer, two resource-backed
XSPICE models, a native BJT, capacitor, inductor and SDT behavioral source. A later
HDL refusal, a later XSPICE refusal, and a native behavioral refusal preserve
native accepted histories, resource state, projected voltages and pending effects.
A retry commits the 0.75 V projected capacitor/inductor state and the corresponding
SDT area once, despite a valid capacitor cache supplied for the original 1 V
candidate. The existing thermal constant-power check covers native-only and
XSPICE engine routes, and the native BJT regression covers Trap/Gear lead-current
and KCL behavior. The fourth check covers the specialized-loader correction above.
The loader correction was pushed separately; the tested combined implementation
is unchanged between these two commits. No additional broad test run was made.

This closes the premature native-history promotion in the two accepted-interval
tails. It is not a complete circuit transaction: solver-controller time/grid and
breakpoint consumption, initial-state orchestration, evaluated native cache
rollback, result-retention failures and effect publication remain outside this
barrier. Irreversible providers still need coordinated effect handling. The new
joint fixture supplies candidates and is not the required full feedback solve
with a shared HDL/XSPICE event net. Typed connectivity, one event/precision
coordinator and root/retry coordination, plus every remaining MS00–MS15 milestone,
remain in scope. No compiler/schema changes, generator refresh, platform or
performance qualification, full release suite, or vendor-reference execution
was performed. Production readiness and Spectre parity remain unproven.

## MS04 increment: retain event types and validate completed connections

The circuit node table now records digital, real, or separate digital and real
representations. Registration combines the domains instead of overwriting an
earlier port's type. XSPICE scalar, vector and inverted digital connections
record their actual value family, and late ground remapping reconstructs that
information. The existing auto-bridge route can attach both event representations
to one electrical node; this remains two scheduler value maps and separate
converters, not a digital/real resolver or an implicit cast. The matrix and shunt
consumers retain their existing event-membership projection.

Mixed HDL boundary validation now runs after every authored and generated XSPICE
instance has been constructed. Previously it read a partial table while building
each X-card: placing the HDL instance before its XSPICE peer could bypass the
shared-event-net refusal and reach an electrical bridge instead. The completed
connection check identifies the HDL instance, port, node and actual event type
independently of card order. It reads wiring before digital startup and does not
need to sample or execute the model to identify its connections.

Five focused checks passed on the combined `abf8ad1bc` baseline (which includes
the concurrent parameter-scoped initial-condition and semiconductor-assignment
fixes). The expanded mixed-route check covers digital and real event connections
in both card orders and requires the same diagnostic. The new typed-representation
case covers two input families on a loaded electrical node in both orders, with
explicit and automatically chosen ground, while preserving each generated
converter. Existing late-ground-remap, event-row pinning and loaded HDL output
checks also passed. Compilation took 1 minute 42 seconds; the selected test bodies
took 0.05 seconds together. No full-suite or platform run was added.

This is connection metadata and a construction-order correction. Direct
HDL-to-HDL/XSPICE event wiring is still open: enabling it requires shared driver
resolution, process regions, timing and circuit rollback rather than simply
removing the guard. Event identities are still coupled to the deck/MNA node
numbering. This increment does not close MS04 or MS05, and does not establish
production readiness or Spectre parity. The user confirmed that no licensed
reference installation is currently available; implementation continues while
vendor qualification remains unavailable.

## MS04/MS05 increment: link HDL instances into one executable digital domain

The compiler now exposes `link_digital_plans`, which combines immutable instance
plans and resolved whole-port connections into one sealed canonical plan. Net
ports collapse into common signals while each continuous driver keeps a distinct
identity. Variable outputs keep their storage and drive the connected net through
an ordinary continuous process; variable inputs use an input copy. Compatible
packed widths may use different ascending or descending declaration ranges:
reads retain their local bounds and write targets map to normalized positions.
Digital and real resolution domains remain distinct, including the existing
single-driver and explicitly selected real-resolution policies. Partial-port and
width-converting connections still require elaborated adapters.

The linked design has one precision and process-region authority. Each process
retains its module time scale and delay-conversion operations, while the design
uses the finest linked precision. Signals, processes, drivers, analog probes and
source-file IDs acquire deterministic design identities. The returned instance
maps retain original-to-design identities, connection-process ownership and source
file provenance for the circuit analog adapters and debugger. Qualified signal
names and collapsed net aliases remain addressable; instance/net input order does
not change the sealed plan. Linking validates the inputs and result and supports
cancellation without mutating the input plans.

`CompiledDigitalDesign::link` and `link_with_control` execute the result through
the existing `DigitalHost`, rather than building another event interpreter.
Linked designs can themselves be instantiated, retaining collapsed aliases.
This is an executable digital integration path and the linker needed by circuit
elaboration. The current `.va`/X-card mixed adapter has not yet migrated its owned
digital state into this common domain, and its direct-XSPICE-connection guard
remains in place. Removing that guard before common scheduling and rollback are
wired would still be incorrect.

Five focused execution cases passed initially on `8c83eca1a` and again on the
combined `cfcc01897` baseline after the concurrent physical-nature/schema and
positional semiconductor-parameter changes. The first compilation caught a test
fixture supplying probe-ID/value tuples to an API that takes an indexed value
slice; the corrected fixture passed on its first execution. The cases verify:

- Two separately compiled samplers exchange their previous values on a common
  clock edge, so all samples precede any nonblocking update. Reordered instance,
  net and endpoint lists produce the same plan and trace; nested linking retains
  aliases.
- A 1 ns/100 ps clock and a 10 ps/1 ps delayed sampler retain their own units on
  a shared 1 ps grid. Restoring a shared host snapshot preserves the delayed
  cross-instance update at 750 ps and the later update at 1750 ps.
- Multiple partial drivers preserve bit order across ascending/nonzero ranges,
  produce X on opposing drives and Z on release. Two real contributions resolve
  to the expected sum under the explicitly selected sum policy.
- Computed event sensitivity and a captured nonblocking assignment waiting for
  two later clock edges retain their relocated dependencies and captured value.
- Two instances of an analog-reading process see independently supplied probe
  values and retain distinct source identities. Invalid single-driver real-net
  connections, unknown ports and cancellation are refused.

The port compatibility and deferred-update requirements are recorded in
[VAMS-2023 sections 6.5.7 and 8.5.3.4](https://www.accellera.org/images/downloads/standards/v-ams/VAMS-LRM-2023.pdf).
These are independent expected-value fixtures, not vendor comparisons. The final
selected test bodies took 0.01 seconds after a 1 minute 4 second build. No broad
suite or platform run was added. The authoritative generator regenerated all 43
built-ins after the final compiler inputs were stable; only its manifest identity
changed, preserving the concurrent published-model updates. Generator digest:
`40c90aa3be3cfd31644a5c5761998599c7fc8e96ab04e4827cca5c5e13a91609`.
The later core-only semiconductor-IC binding commit `d3c1db7af` was incorporated
without changing this compiler/runtime path or its generator inputs; the focused
checks were not repeated for that unrelated integration.
This API addition changes neither the existing artifact wire format nor existing
source lowering, so it adds no schema/cache-format bump beyond upstream schema 50.

The required whole-circuit HDL/XSPICE/loaded-SPICE example, circuit-wide root and
retry coordination, platform/backend qualification and every remaining MS00–MS15
requirement remain open. The host snapshot fixture verifies shared digital replay;
it does not qualify complete circuit rollback or persisted restart. Native/JIT
and Wasm digital execution are not newly qualified by this portable runtime path.
Production readiness and Spectre parity remain unproven, with no licensed vendor
reference currently available.

## Circuit ownership of linked mixed HDL execution

Circuit elaboration now enrolls all mixed HDL instances into one running
`DigitalHost`. Each analog model keeps its immutable local signal metadata and a
value view; it has no independent process resumptions, driver contributions or
event queue. Instance relocation maps connect these views to the linked plan.
The circuit uses the finest participating precision while each process retains
its declared delay unit. The shared scheduler honors the strictest participating
configured event/delta limits.

Newton probes, candidate control-task inspection and final acceptance open all
model trials together. Every digital analog-potential probe is populated before
due process execution. All A/D decisions from the candidate are then published
as one bank before dependent processes run; analog equations read the resulting
values only after settlement. Separate scoped guards restore the shared queues
and every participating model on errors or numerical rejection. Acceptance
prepares every model before promoting any model or the shared digital state.
Circuit clones include the shared state, and successful analysis reinitialization
resets it with the model views. An individual enrolled model's checkpoint or
external force is refused because neither can represent a circuit-wide update.
Standalone mixed-host APIs retain their existing owned-runtime behavior.

The production circuit builder, operating-point/transient stamps, scheduled
breakpoints and the existing native/external acceptance barrier use this path.
This change does not yet attach XSPICE drivers to the shared HDL nets: the
existing explicit refusal of that connection remains. Analog boundaries retain
their physical D/A source resistance and external loads.

The focused native/HDL/XSPICE acceptance refusal/retry case passed before
integration and again on `f5742a7fe`, after the concurrent BJT junction-geometry
change. Five existing circuit cases passed: initialization finish before digital
execution, internal potential/branch wiring, analog oscillator counting, a packed
port above bit zero, and accepted digital replay across rejected timesteps. The
new two-instance circuit passed in both deck orders on the combined revision:
independent initial and edge-triggered analog reads, loaded 20-ohm D/A sources
against 1-kohm resistors, one shared 1-ps grid, and retained 500-ps/25-ps process
delays. The final two selected test bodies took 0.07 seconds after a 1 minute
6 second build. Unaffected passing circuit cases were not repeated for the
core-only BJT integration. Logs in the owned target directory:
`circuit-digital-acceptance.log`, `circuit-digital-route.log` and
`circuit-digital-combined.log`.

The new circuit timing fixture also made an existing limitation explicit: a
continuous A/D threshold crossing can be delivered at the next accepted analog
sample. In its 20 ps-step ramp, the mathematical 0.95 ns crossing was delivered
at 0.97 ns. The initial assertion expecting an output exactly 0.5 ns after the
mathematical crossing failed. The shared-grid oracle now separately bounds that
sampling delay and checks exact scheduled delay from the recorded delivered
edge. This establishes circuit precision and retained process units; it does
not establish exact threshold localization. MS03's candidate root refinement
must remove that delivery-time dependence rather than treating this fixture as
qualification of crossing accuracy.

No compiler schema, generator inputs or generated device bodies changed. No
vendor, full-suite, backend or platform qualification was performed. The user
confirmed that no licensed reference installation is available. Spectre parity,
production readiness, persisted mixed restart and all incomplete MS packages
remain unproven/open.

## Resolve mixed A/D roots before accepting their consequences

The mixed HDL boundary now requests the transient controller's existing
candidate rejection/refinement path. During candidate inspection it checks all
A/D boundaries using the accepted digital level, accepted voltage and converged
candidate voltage, and returns the earliest physical threshold root before
opening another candidate HDL trial or delivering control tasks. It does not evaluate analog stateful operators again.
The controller restores speculative state and solves at that root before any
digital consequences are accepted.

Root time remains in seconds; it is not rounded to the HDL tick grid. The
effective time tolerance is the greater of 64 machine epsilons times the
absolute time scale and the active solver's hard minimum timestep. A target
extremely close to the accepted point advances by the first representable
interval meeting that minimum. This removes requests for illegal subminimum
steps while bounding delivery error by the solver's numerical capability.
Zero-width thresholds use the entering direction at equality, retain the held
level while stationary, and preserve the existing zero-at-threshold startup
convention until an accepted voltage history exists.

The shared crossing interpolation also no longer treats tiny finite voltage
changes as flat or lets overflowing finite voltage differences fall back to
the endpoint. Scaled, bracketed interpolation preserves the crossing fraction
for both extremes and is reused by existing XSPICE callers.

The previous increment's one-analog-sample allowance is removed from the timing
regression. The 0.95 ns ramp crossing is checked against its analytical time
with a declared 2e-20 s bound (twice the default solver hard minimum), for
20 ps and 75 ps maximum steps and both instance orders. Its 500 ps and 25 ps
delayed updates still land on the shared 1 ps grid. A new sinusoidal fixture
checks concave and convex crossings in both directions against their analytical
roots and checks the delayed output time. The existing oscillator counter again
records all ten cycles, including the first edge after initialization.

Four focused interpolation cases, including tiny/extreme finite values, and the
existing rejected-timestep digital replay case passed on the initial run.
That run caught requests below the solver floor and an unintended startup edge;
both were corrected. The three affected circuit cases then passed on
`862ec6491` plus this change; their test bodies took 0.13 seconds after a
20 second build. Logs: `mixed-adc-roots.log` and `mixed-adc-root-floor.log` in
the owned target directory. No compiler/generator inputs, broad suite or platform
checks were added. The concurrent BJT branch-current change is integrated
separately; these resistor/source/HDL fixtures do not exercise those BJT equations.

This closes the observed sampled-delivery defect for the mixed HDL electrical
A/D adapter within the stated time tolerance. It does not qualify arbitrary
unbracketed/non-monotonic event detection, XSPICE boundary root refinement, flow
or analog-owned-variable reads, full feedback convergence, reference equivalence,
or the whole MS03 package. The full approved plan remains active.

## Direct HDL event bits and final circuit node identities

HDL-only deck nodes now connect the original compiled wire-driver contributions
inside the circuit digital runtime. Whole-vector publications update every
connected alias before expression sensitivity observes the value bank. Aliases
never re-enter resolution as additional drivers, so contention and high-impedance
release propagate without resolved-value feedback. Variable ports retain the
linker's explicit continuous connection processes, including input-variable
copies; they are not collapsed into wire storage.

The builder distinguishes physical use from discrete port use. Terminal
connections and compiled behavioral voltage references retain their electrical
converters and loads; an HDL-only bit uses the event graph. One vector can contain both. A/D
publication updates only its physical bits on the global port wire, preserving
the event-connected bits. Fresh analyses retain immutable bit topology while
resetting values and execution state; speculative graph values and original
driver contributions remain inside the existing circuit rollback image.

Review also found that automatic ground selection follows mixed elaboration.
It previously omitted the mixed host's node references. Remapping now includes
analog terminals, internal potentials and allocated branch-current unknowns,
analog-read probes, electrical bridges, event identities and bus members. The
net-kind cache is rebuilt from both XSPICE and the HDL graph afterwards.

Validation covers opposite vector directions and nonzero ranges, partial-vector
co-drivers with contention/release, a partly physical input variable, explicit
and automatically selected ground, a loaded SPICE output, and replay of a split
HDL clock/divider through actual analog timestep rejections. The initial six
selected cases passed after declaring the fractional-delay fixture's intended
1 ps precision. The expanded route target passed 28 of 30 cases; two new oracle
assumptions needed correction: SPICE canonicalizes node names, and an f64 stop
time can lie just before the physical time of the nominally coincident digital
tick. The replay fixture ends between events and checks interior activations;
this does not close MS02's final-time/rounding qualification.

After integrating remote main through d3b7ed56e, all 30 mixed-route cases passed
in 0.99 seconds following a 70 second dependency build. That baseline includes
the concurrent shared-process diagnostic, indirect-equation tolerance, residual
scaling and BJT fixes. Subsequent review added classification of compiled
behavioral voltage references; its new voltage/current-source observation case
and the two affected direct/partial-vector cases passed in 0.09 seconds after a
21 second build. Logs in the owned target directory: hdl-event-bit-graph.log,
hdl-event-bit-graph-timing.log, hdl-event-bit-graph-route.log,
hdl-event-bit-graph-integrated.log and hdl-event-bit-graph-observation.log. The
full workspace suite and platform/release checks remain deferred to their
integration milestones as requested.

The graph currently covers four-state HDL wire bits and retains empty MNA rows
for stable deck node identities; removing those unnecessary unknowns still
belongs to MS04. Direct HDL/XSPICE connections are still refused until driver
strengths and regional interleaving share the circuit authority. This is a
required intermediate implementation, not completion of the planned two-HDL /
XSPICE / loaded-SPICE / off-grid / rejected-trial slice. No compiler or generator
inputs were changed by this increment. Reference, full-suite, backend, platform,
capacity and commercial qualification remain open.

## Cooperative event regions and external driver contributions

The HDL host now has a restricted Active-participant interface. An enrolled
participant can consume resolved-net changes and publish its declared drivers;
it cannot advance HDL time or promote an HDL region. The host dispatches that
work and its HDL consequences before advancing inactive or nonblocking work.
External activations count against the existing scheduler limits, retain their
own diagnostic identities, and can use the physical causal lane without
consuming an unrelated future timer at the same rounded reporting tick.

Connected-bit topology now retains each external output contribution, its
stable identity, and original HDL contributions separately. It resolves the
existing XSPICE state/strength values in deterministic driver order and projects
the resulting logic level into HDL wire views. A compact change journal retains
intermediate resolved values, strength-only changes and boundaries between
atomic vector publications. Fresh runs preserve immutable connections and
reset drivers, observations and process state. A missing participant or late
topology edit is refused before a standalone call can write inputs or execute.

The production XSPICE evaluator now uses a resumable Active-wave step. Its
ordinary entry points call the same step until quiet. Current-source samples,
analog transition observations, analysis phase and companion coefficients
survive a yield; consumed pending work is cleared. Shared-net observations
update input views and dirty/pending fanout without becoming additional output
drivers. This supplies the two scheduling sides needed by the circuit adapter.

Focused validation on d51a5042d plus this increment passed six unit cases and
three existing mixed-route cases. The unit cases include a real XSPICE inverter
returning its time-zero response before HDL inactive/NBA work, explicit failure
and clone/replay of both runtimes, strong contention/weak-pull release, fresh
state, atomic vector publication without a computed-sensitivity glitch, and an
off-grid activation that leaves its future timer pending. Existing XSPICE
model-error and oscillation checks also passed. The selected deck cases retain
direct HDL vector resolution, partly electrical input variables and replay
through actual rejected analog steps. The final run took 0.74 seconds in test
bodies after a 78 second build; log: hdl-xspice-active-waves-final.log in the
owned target directory.

Fixture corrections did not change model policy: the inverter still imposes
its ngspice-compatible minimum propagation delay after startup, and its
zero-time response is what the regional test exercises. Module-level integer
ownership remains an MS06 gap; the atomic-bank fixture uses a packed register.
Earlier compiler/fixture failures and focused reruns are retained in the
hdl-xspice-active*.log files. No full-suite, vendor, platform, generator or
capacity qualification was performed.

The production circuit adapter is still required: builder enrollment of actual
XSPICE ports, shared resolution during XSPICE event drains, a common speculative
and accepted trial across both families, queue breakpoints and one accepted
snapshot per shared node. Direct HDL/XSPICE deck connections therefore remain
refused. The private attachment APIs currently produce unused-code warnings in
a non-test build; circuit integration must make them live rather than suppress
those warnings. This increment is scheduling/driver infrastructure with runtime
execution evidence, not the completed whole-circuit slice or Spectre parity.

## XSPICE event drains through shared HDL resolution

The circuit adapter now enumerates actual XSPICE digital output ports, retaining
scalar, mapped/inverted and vector-element driver identities. Its immutable
binding table enrolls only XSPICE endpoints on the offered HDL bit groups.
Input observations never become output drivers. Shared event drains publish
original contributions into the HDL resolver and immediately make the combined
resolved value available to native XSPICE fanout. Nonshared digital and real
nodes retain their existing event-drain route.

The borrowed Active participant consumes resolved HDL publication banks and
runs the production XSPICE dispatch one wave at a time. XSPICE vector output
banks are published atomically; the original contributions remain separately
available for rollback and diagnostics. Physical execution time stays distinct
from the rounded HDL reporting tick, and a missed XSPICE breakpoint is refused
before its event queue is drained.

The initial focused run passed six cases in 0.07 seconds after an 81 second
build: two actual routed-inverter cases, the two existing original-driver drain
cases, and existing model-error and oscillation refusals. The routed cases check
HDL contention reaching native XSPICE fanout, release before HDL inactive reads,
and the real gate's 1 ps propagation from a 100.6 ps activation without consuming
future HDL timers. They also replay cloned digital and XSPICE state. A further
vector case checks mapped output inversion, independent original driver indices
on one conductor, release, and atomic vector expression observation. Its first
compile needed a fixture correction to unwrap the declared optional digital
input. After integrating remote main through a45d90faf (including the Verilog
last_crossing/runtime/generator update), all three routed cases passed in 0.01
seconds after a 109 second build. This includes vector and missed-breakpoint
checks. Review also removed a redundant model-error wrapper from the two event
drain refusal paths, preserving their original error types and diagnostics.
The two affected model-error/oscillation cases then passed in 0.07 seconds after
a 31 second build. Logs: hdl-xspice-routed.log, hdl-xspice-routed-final.log,
hdl-xspice-routed-integrated.log and hdl-xspice-routed-errors.log in the owned
target. The increment changes core and documentation only; generated artifacts
came from the concurrent last_crossing commit and were not regenerated here.

This is the actual event-drain adapter, but it is not yet attached by the deck
builder and joint circuit trial. Direct HDL/XSPICE decks remain refused. The
next change must retain candidate-local code-model state across Active waves,
wire builder enrollment and circuit rollback/acceptance, include both queues'
breakpoints, and publish one accepted shared-net snapshot. The existing
RollbackableProbe phase intentionally suppresses some code-model state changes;
it must not be relabeled as an accepted evaluation merely to enable feedback.
The private adapter APIs remain unused in a non-test build until those circuit
paths are connected. No full-suite, vendor, platform, generator or capacity
qualification is claimed by this increment.

## Joint HDL/XSPICE numerical candidates

Circuit numerical probes now retain the XSPICE candidate through shared HDL
settlement and through both families' matrix/RHS stamps. A circuit-owned probe
guard temporarily separates the HDL coordinator and model views from code-model
storage, restores each owner on exit, and rolls back the XSPICE COW model/event
images and registered external resources after the whole evaluation. A later
analog stamping refusal therefore cannot leave an earlier event participant or
registered provider advanced.

The new CircuitTrial evaluation phase preserves candidate-local code-model
memory, output events and dirty-input signatures across Active waves. Existing
RollbackableProbe behavior stays intact for standalone single-call probes;
CircuitTrial requires an owning resource transaction. Reversible d_cosim
providers continue within that outer transaction. External d_process and
irreversible d_cosim providers cannot execute a circuit trial without a rollback
protocol; silently skipping their feedback or treating a probe as accepted would
not implement the coupled contract. Their full effect/protocol support remains
part of MS05.

Circuit digital finalization now enrolls actual XSPICE endpoints in its shared
bit groups, with failure restoration of the previous HDL owners. Ground-driven
node renumbering updates the binding table and diagnostic driver identities;
fresh HDL analyses retain immutable attachment identities. A shared candidate
also invokes the external Active participant when no HDL timer is due, using
physical time and the causal lane. The companion policy, including Xyce
OneStep weighting, is passed explicitly into the joint numerical stamp.
Independent XSPICE stamping and OP/UIC seed evaluation are bypassed for a
coordinated event circuit so they cannot separately advance or double-stamp its
candidate. Actual deck connections are still refused until acceptance and
candidate-control inspection are integrated.

Focused evidence uses two circuit-enrolled HDL instances, the built-in XSPICE
ADC and inverter, native SPICE resistors, a retained 20 ohm D/A output and a
registered mutable provider. ADC-triggered feedback settles before an HDL
inactive read and returns through an NBA update. Alternating numerical
candidates and a late analog sqrt-domain refusal restore both runtimes, views,
queues and provider state. One provider undo image covers every Active wave.
The loaded output retains the analytic 1000/1020 voltage ratio. This exercises
the production circuit probe and native stamping APIs at startup; it is not
an end-to-end accepted transient trajectory or the completed off-grid/rejected-
timestep deck slice.

After correcting a stale enrollment call site and constructing the fixture
through the builder's unstarted compiled-host entry, the joint case passed in
0.02 seconds. The two existing direct-HDL/rejected-timestep deck cases passed
in 0.65 seconds after the same 31 second build. Three earlier routed-adapter
cases had also passed. Three legacy gate/ADC/d_process probe cases passed with
no rebuild in 0.00 seconds. The reversible d_cosim candidate now has focused
success/failure evidence: successive Active calls retain provider state until
outer rollback, including a failed call, and legacy single-call probe behavior
still restores immediately. Both provider cases passed in 0.00 seconds after a
35 second build. Logs: hdl-xspice-joint-probe.log,
hdl-xspice-joint-probe-checked.log, hdl-xspice-joint-probe-fixture.log,
hdl-xspice-joint-probe-legacy.log and hdl-xspice-joint-probe-cosim.log in the
owned target directory. No blanket unused-code suppression was added; the
reporting-tick inspection helper is now test-only. No compiler or generator
inputs were changed by this increment. Full-suite, vendor, platform, capacity
and commercial qualification remain open.

## Accepted shared HDL/XSPICE circuit events

Direct digital-only HDL/XSPICE deck connections now use the circuit coordinator
through startup, numerical probes, candidate-control inspection and acceptance.
Native history preparation runs after shared event settlement, code-model voltage
projection and HDL candidate validation. Every HDL acceptance is reserved before
native promotion; a late preparation refusal drops all reservations and restores
the shared scheduler, instance views, XSPICE models/queues, registered resources
and projected solution entries. The existing native acceptance implementation is
shared by the connected and independent paths. Standalone code-model evaluation
refuses enrolled drivers that require the circuit participant.

The builder permits shared digital-only conductors after continuous-node
classification. It continues to refuse a shared conductor with physical loading
or an unlike real event domain, which still requires the MS04/MS07 conversion
contract. Physical HDL D/A loads elsewhere in the circuit remain stamped. The
accepted snapshot comes from the common resolver exactly once per shared node
and preserves the resolved XSPICE drive strength.

The whole-deck check exposed two execution gaps, both corrected here. Origin-time
acceptance used the old standalone HDL path. It now uses the joint barrier.
Additionally, an instantaneous HDL D/A change was missing from candidate
integration-discontinuity reporting: adding XSPICE selected generic voltage LTE,
which then repeatedly rejected the finite output jump against smooth history.
Candidate reporting now compares each retained bridge's actual final driven level
and impedance with its accepted value, including Z release. Pure event-only bits
and intermediate delta glitches with no final electrical change add no bridge
discontinuity. The same level mapping drives stamping. Both event queues retain
an exact absolute pending target outside the general breakpoint manager's
coalescing tolerance; interval comparison avoids subtraction/addition drift.

Focused evidence: a late injected native-preparation refusal after real ADC/gate
settlement and mutable-provider evaluation restores all owners/resources on two
attempts, followed by successful retry and one sorted snapshot entry per node.
The earlier joint numerical probe also passed. The initial library filter matched
25 cases (including older coupled-adapter and unrelated names); all passed in
0.02 seconds after a 46 second build. Subsequent selections are explicit.

The actual deck contains two HDL instances, the built-in delayed inverter and a
native 1k/10p RC load. Coarse and fine runs agree on all 41 clock/inverter and 21
divider trace points, with actual coarse-run timestep rejections. The first
post-start rising inverter event at 10.1006 ns captures a zero before a distinct
HDL timer at 10.101 ns; both retain their physical timestamps. All 40 interior
inverter events match the authored 100.6 ps delay within 2e-20 seconds. The loaded
analog output has a nontrivial RC waveform. This deck passed in 2.70 seconds after
a 23.51 second build. Logs are hdl-xspice-acceptance-joint.log and
hdl-xspice-acceptance-discontinuity.log in the owned target. Earlier acceptance
logs record compiler/startup/discontinuity failures during development; temporary
step diagnostics have been removed.

After integrating remote main through 48ec49197 (the concurrent BJT transport/
external-BC charge change), the joint probe, shared acceptance-refusal/retry and
native-history barrier cases all passed in 0.02 seconds after a 51.57 second
build. The shared off-grid deck and loaded/unlike-domain refusal passed in the
three-deck selection after a 34.20 second build. The older direct-HDL fixture
then correctly reported no rejections at its former 1 ns ceiling: its finite
D/A edge no longer triggers pointless retries. Increasing only that fixture's
coarse ceiling to 4 ns exercises native RC curvature/truncation; it passed with
actual rejections and unchanged trace assertions in 0.63 seconds after a 1.60
second test-only build. Logs: hdl-xspice-acceptance-integrated-barrier.log,
hdl-xspice-acceptance-integrated-decks.log and
hdl-xspice-acceptance-native-curvature.log. All six selected properties now pass.
No full-suite or platform run was performed for this increment.

Two additional findings remain open in the full plan. MS06 must handle the valid
mixed numeric comparison in "$realtime > 0": the compiler refused its integer
literal operand; the scheduler fixture uses an explicit 0.0 operand and nested
conditions. MS05 controller integration must prevent breakpoint step equalization
from proposing half an interval below the model integration floor; the failed
edge approach reproduced 8.4173899133396096e-21 seconds against a 1e-20 floor.
The electrical-discontinuity fix removes that failing approach from this deck,
but does not itself close the general controller-floor requirement. Mixed Xyce
static-history assembly, all-owner startup/history consistency, continuous XSPICE
root refinement, irreversible provider effects and general feedback re-solving
remain open. This increment does not qualify advanced analyses, generated/Wasm
execution, platforms, capacity, production readiness or Spectre parity. No
licensed reference installation is available, as confirmed by the user.

## Model-floor-aware adaptive event approach

Adaptive transient interval fitting now considers the mandatory event/stop time,
the model/controller minimum, the persistent user maximum and the current model
bound together. It retains an ordinary proposal when the remaining interval can
still be covered by supported steps. Otherwise it chooses a feasible interval,
including direct event landing when two minimum-sized steps cannot fit. It does
not enlarge the model/user ceiling, move the event or accept a rejected solution.
An impossible subdivision reports its times and bounds before model evaluation.
Locked replay retains its prescribed intervals and existing refusal behavior.

The fitting calculation runs after source biasing and breakpoint equalization,
so those operations cannot reintroduce the recorded subminimum half-interval.
The final candidate retains the exact shared-device target when fitting reaches
it. A current model bound is not assumed to remain fixed for future candidates;
the partition feasibility ceiling is the persistent user maximum. Existing
canonical-stop roundoff handling remains available for fixed minimum-step runs,
while a distinct scheduled device event retains its exact interval contract.
Fused subtraction handles the near-integer interval-count rounding that initially
refused a previously supported 100-step minimum-interval run.

Focused evidence constructs a real analog timer at 2.5e-20 seconds with a
1.25e-20-second maximum and a 1e-20-second model floor. Ngspice and Xyce land
exactly on the event, preserve its before/after output and keep all recorded
integration intervals within the bounds. The fixture explicitly reports its
instantaneous output discontinuity. Existing minimum-bound/maximum-floor runs and
unsupported maximum/locked/final intervals also pass: three selected deck cases
in 0.11 seconds after a 19.30-second build. Initial failures and the corrections
are in model-floor-interval.log and model-floor-interval-rounding.log.

Integrated remote main through fed97cb86, preserving c67b90fb5's runtime/backend
integral-range/circular-phase change and regenerated models, and the other
agent's updated legacy mixed rollback fixture. The interval feasibility unit
passed in 0.00 seconds after a 108-second dependency build; it covers the exact
recorded failing interval, a feasible constrained split, a conflicting bound,
a changing model bound, an unsupported interval and an ordinary distant target.
All three selected timestep deck cases passed in 0.07 seconds after a 63-second
build. The shared HDL/XSPICE off-grid/rejected-step deck then passed in 2.74
seconds after a 1.91-second test build. Logs: model-floor-interval-integrated-unit.log,
model-floor-interval-integrated-decks.log and model-floor-interval-shared-deck.log.
The final normal push first encountered concurrent main commits 1467c12c2 and
686717809. Rebased without conflicts, preserving their BJT current-balance and
thermal/junction-noise work. They do not change these timestep APIs or the
devices in the selected fixtures; the same focused tests were not rebuilt again.
No full-suite, generator rerun or platform qualification was performed here.
This completes the recorded equalization defect; it does not close the remaining
controller, initialization, cache/effect/result, root or static-history work.

The next static-history change must distinguish physical static contributions
from ordinary residual probes. Current VM StaticProbe disables limiting but has
no StaticDaeProbe mode, while generated models expose a separate dynamic-operator
policy. The core capture calls the ordinary VM probe, and mixed hosts are absent
from capture during shared acceptance. Do not simply stamp mixed transient
companions into static history or reuse a DC analysis: both would change the
intended transient equations. Derivative/integral state and weighting need direct
analytic evidence alongside a settled digital candidate before MS08 can close.

## Runtime static DAE integration observations

Added an explicit runtime StaticDaeProbe policy corresponding to the generated
model policy. It retains the physical analysis/time and bypasses Newton limiting.
The VM and native ddt helpers return zero without proposing integration history;
their derivative and integral Jacobian actions are zero. Integral operators still
validate their authored operands, then retain the valid current integral candidate,
fall back to accepted history when there is no candidate, or use the initial
condition before either exists. In particular, disabling integration alone would
incorrectly replace a nonzero transient integral with its initial condition.

Device value and stamping entry points isolate this mode in a runtime-state copy,
sharing compiled code. Successful and failed observations leave the original
candidate, histories, event variables, effects and convergence state unchanged.
Beginning the observation preserves current integration-valid flags and event
variables and disables task recording. Native helpers receive an appended context
flag; existing generated-code offsets are unchanged and the context layout check
covers the new size/offset. Native image caching is process-local; the persisted
model-state shape is unchanged. Emitted WebAssembly uses the same VM operator
policy through its existing runtime helpers and does not change its frame layout.

The three focused regressions passed in 0.01 seconds after a 105-second build
with native and wasm-jit enabled. A real native device exercises two successive
transient candidates, nonzero idt and wrapped idtmod values, a nonzero second-step
ddt term, physical analysis("tran"), repeated value/stamp observations and a late
invalid contribution with no published matrix callbacks. The VM operator case
also covers accepted-history fallback and invalid frozen operands. Independently
emitted Wasm automatic/postfix plans retain the earlier integral candidate after
a changed voltage and return the static Jacobian. This is host execution in wasmi,
not a shipping-browser run. The existing exact native context-layout check passed
in 0.00 seconds using the same built test binary. No full suite was run.
Log: target/unified-mixed-fixes/static-dae-probe.log.

The authoritative generator then regenerated all 43 built-ins against the final
compiler inputs. Only the manifest generator digest changed, to
99f6c8e098e9311cf0d713f54c73537bd7d3b57acc7a5e3adf6da2417fd7de37.
Log: target/unified-mixed-fixes/static-dae-generator.log.

This is the runtime integration-observation prerequisite, not complete MS08
circuit history assembly. The core capture still uses ordinary StaticProbe; shared acceptance
still needs a settled mixed candidate available for static observation. Event
body re-entry and filter/static decomposition need qualification before that
connection. Runtime and mixed OneStep2 derivative weighting also remain open;
integral operators need the correct previous-input term independently of the
weight applied to ddt contributions. No full-suite, shipping-platform or licensed
reference qualification is claimed by this increment.

## Static observations retain settled analog event state

A static observation must not run an analog event body a second time on its
retained candidate. Runtime StaticDaeProbe now suppresses initial/final-step,
timer, cross and above event guards while preserving the physical analysis and
phase. Cross/above observations still validate their numerical inputs and
tolerances, without allocating or changing detectors or requesting a refinement.
Timer observations cannot fire or publish another time bound. Native x64 and
AArch64 global-event code reads the observation policy explicitly; VM/Wasm query
masks suppress only the global event flags. Physical analysis queries remain
unchanged.

Generated Rust now reads procedural event variables from the candidate during a
static observation, suppresses event firing, and avoids publishing event-variable,
detector and timer updates. The core generated-device observation no longer
resets event state from accepted history before stamping. Its existing rollback
snapshot still restores all device state after either successful or failed
observation. Ordinary Newton, residual-probe and small-signal lifecycle behavior
retains the existing event evaluation policy.

One shared authored model exercises five independent event counters: initial,
final, timer, cross and above, together with static conductance and ddt current.
Its exact static currents are -1 at startup and 11113 after the scheduled rising
candidate; repeated static observations retain those values without replaying the
counters. The native device test additionally verifies the static Jacobian and
unchanged original context. The VM case checks frozen guards, physical tran
identity and invalid tolerance diagnostics. Wasm automatic and postfix plans run
in wasmi against the same model. Their harness now allocates detector/event state
and transfers the variable array and analysis mask just as the primary worker
dispatch does; the initially missing harness storage/mask produced the recorded
failures, rather than successful qualification. The corrected Wasm event case
passed in 0.01 seconds after a 6.86-second test build. Two generated-Rust static
observation cases passed in 0.61 seconds after a 31.95-second build. The initial
six-test VM/native/Wasm selection passed five cases; only the Wasm fixture needed
those harness corrections. No full suite was run.

Logs: static-dae-events.log, static-dae-events-wasm.log,
static-dae-events-wasm-frame.log and static-dae-events-generated.log, all under
target/unified-mixed-fixes. The actual generated runtime policy check passed in
0.00 seconds after a 55.79-second dependency build; it keeps physical tran/scope
queries true while suppressing the two global-event flags only in StaticDaeProbe.
The native x64 emitted-query check passed in 0.00 seconds after a 7.52-second
build, covering all query IDs, physical analyses, phases and initial/final flag
combinations with and without static observation. The corresponding AArch64 case
is updated for its platform run and was not executed on this x64 host. Logs:
static-dae-event-mode.log and static-dae-event-analysis.log. The authoritative
generator regenerated all 43 built-ins; 35 stamp files and three noise files now
select the correct procedural-state bank, with their manifest hashes updated.
Generator digest: 2c14fba97c54558578e51e4e4475399096eaf15a3d89448bbaf5e9e092f4deda.
Log: static-dae-event-generator.log. Rebased onto concurrent main e94cdd760,
preserving its BJT base-resistance/temperature-control changes. The initial core
selection compiled but ran zero cases because VBIC13 is not a default feature;
static-dae-event-core.log is build evidence only. With veriloga-model-vbic13
explicitly enabled, generated_static_dae_probe_restores_event_candidate_bitwise
passed in 0.00 seconds after a 110-second build. It exercises the production
adapter and preserves all accepted/candidate numeric bits and flags. Log:
static-dae-event-core-vbic.log. No additional broad test run was performed.

This closes event-body re-entry for these static-observation routes. MS08 still
requires filter/static decomposition, correct runtime/mixed OneStep2 derivative
and integral weighting, and a settled shared candidate in complete circuit
static-history capture. Actual AArch64, browser-worker and tablet execution
remain platform qualifications; this native-host evidence does not replace them.
No licensed reference installation is available and Spectre parity is unproven.

## Mixed models retain the complete transient companion rule

The production OneStep eligibility guard excluded runtime analog-only models,
but omitted mixed hosts. With a mixed conductance, the solver therefore weighted
native static terms by one half and added their accepted history, while stamping
the mixed conductance in full without its accepted static history. The same
route handed mixed integration operators the OneStep backward-Euler rule.
An actual deck reproduced a 2.9590614 mV discrepancy between otherwise identical
native and mixed RC branches on a 10 ns locked grid.

The guard now includes mixed hosts. Runtime analog-only and mixed models both
use the ordinary complete companion formulation until the full static/dynamic
split is implemented for them. This preserves ordinary second-order trapezoidal
and Gear integration; it does not force all their steps to first order. Generated
models retain their existing compiler-proven split eligibility.

One focused deck compares a mixed conductance with native capacitance, a mixed
conductance plus ddt capacitance, and an initialized mixed idt against equivalent
native branches at every accepted point. All three agree within 1e-8 V in Xyce
and Ngspice modes. Independent RC-ramp and integral closed forms pass a 3e-4 V
bound that allows first-order startup. The final fixture uses iteration-based
order control on its fixed grid to guarantee promotion to order two; the initial
post-fix run already passed all native-equivalence comparisons, but its default
LTE order selection did not satisfy the analytic integrator bound. The final
focused test passed in 0.05 seconds after a 1.26-second build. The existing shared
HDL/XSPICE off-grid event and rejected-step deck passed in 2.70 seconds after a
1.81-second build. No broad suite was run. Evidence logs are
mixed-transient-weighting-before.log, mixed-transient-weighting-after.log,
mixed-transient-weighting-final.log and mixed-transient-weighting-shared.log,
under target/unified-mixed-fixes.

This fixes the reachable inconsistent weighting route. Complete runtime/mixed
F/Q observation and OneStep eligibility, including retained filter state and
shared accepted static-history assembly, remain MS08 implementation work. The
absence of a licensed Spectre installation remains recorded; these analytic and
native-equivalence checks do not establish vendor parity or production readiness.

## Laplace static observations preserve their settled internal state

A Laplace static DAE observation now returns C*x_candidate + D*u, falling back
to accepted x when no candidate is present. Its Jacobian action is D*du. It
neither re-integrates the changed input nor substitutes a new DC equilibrium.
The shared state-space implementation validates structure and operands without
changing accepted/candidate arrays or flags. VM and native helpers select this
policy explicitly; the emitted Wasm runtime uses the same VM helper. No context
ABI, persisted state shape or ordinary integration coefficients changed.

The shared authored fixture combines a conductance, H(s)=(1+2s)/(1+s), and a ddt
contribution. Its first two backward-Euler internal states are 2/3 and 16/9, so
static current is exactly 4*V-state, static Jacobian is four, and the ordinary
transient Jacobian is 29/3. Actual native and emitted Wasm automatic/postfix
routes exercise both accepted timepoints, repeated observations with changed
input, and unchanged retained filter/integration state. The native route also
checks a late invalid contribution publishes no callbacks and leaves the
original context unchanged. The VM case checks accepted-history fallback,
invalid value/derivative inputs, and a stateless filter's full direct action.

The fixture also exposed a production compiler refusal: the equation's Jacobian
retained a Laplace state read but replaced ddt with a stateless coefficient. The
JIT planner incorrectly correlated that reduced derivative program against all
state operators in the original equation. Jacobian and limiter-correction
planning now obtain the already-renumbered state identities from the equation's
primal program. The valid combined Laplace/ddt equation compiles and executes
through native and Wasm routes. Native automatic selection uses the compiled
postfix plan because Laplace block-model lowering is not yet available; no
interpreter fallback is claimed. Direct generated-Rust Laplace remains an
existing explicit refusal.

The VM observation case passed in the initial focused selection. The shared
fixture first needed the required array assignment-pattern syntax, then exposed
the planner refusal described above; both failures remain in their logs. After
the planner fix, both native/Wasm cases passed in 0.01 seconds. Their final
expanded check of ordinary and static Jacobians passed in 0.01 seconds after a
10.24-second build. No full suite was run. Evidence is in static-dae-laplace.log,
static-dae-laplace-runtime.log, static-dae-laplace-planner.log and
static-dae-laplace-final.log under target/unified-mixed-fixes.

The existing emitted-Wasm nonlinear limiter/correction regression passed in
0.02 seconds using the already-built test binary, covering the other derivative
route that now reuses primal slot identities. Log: static-dae-laplace-limiter.log.
The authoritative generator regenerated all 43 built-ins against the final
compiler sources. Only the manifest generator digest changed, to
58e15821e5f76b3ea1a56671f386f768f6c73a0b88aff1504abc9ba561bee956.
Log: static-dae-laplace-generator.log. No generated model source changed.

This completes the retained state-space observation prerequisite. Other filter
families, complete shared static-history capture, and general runtime/mixed F/Q
integration remain open. The newly corrected companion guard stays in place.
Shipping browser/tablet and licensed-reference qualification remain open.

## Next implementation work

Complete the all-owner startup/history contract and the remaining MS05
controller/effect/result/cache transaction. Preserve physical loads
and converters while implementing shared loaded conductors, RNM and authored
conversions, and remove unnecessary analog unknowns from digital chains. Extend
the root handshake to XSPICE boundaries and complete flow/analog-owned-variable
dependencies and feedback convergence. Close the recorded mixed real/integer
comparison refusal under MS06 and include mixed contributions in Xyce static
history before qualifying that integration path.

Continue hierarchical source/library/view binding and the remaining MS02 delay
and time-declaration semantics alongside those interfaces. Finish the controller,
initialization, cache and effect/result portions of the circuit transaction, then
all remaining MS00–MS15 requirements in the approved plan. Qualify the actual
browser worker and other shipping platforms at their integration milestones.
