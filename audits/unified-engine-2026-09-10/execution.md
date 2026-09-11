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

## Next implementation work

Carry standalone library entries through project-editor and signed-PDK bindings;
those adapters still require an executable module selection. Keep their compile
profiles, terminal contracts and backend qualification consistent with the mixed
model routes. Qualify the actual
browser-worker path at its integrated target milestone.
Continue hierarchical library/view binding with the typed design graph.
Continue MS02 with remaining delay semantics and time declarations,
then MS04 typed graph and the MS05 coordinator. The first circuit-wide slice must
include two HDL instances, an XSPICE participant, a loaded SPICE boundary,
off-grid timing and a rejected trial. Continue through every remaining milestone
in the approved plan; no parity claim is made at this intermediate checkpoint.
