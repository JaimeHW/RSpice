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
| Active source, macro/include, cache and virtual connection closure | MS01 | Artifact transport and shared file preparation implemented; design-wide configuration work remains | Focused file/virtual and source-refresh cases pass |
| Complete resolved module timing and scheduling regions | MS02 | Open | Pending |
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

Design-wide dependency consistency across different roots and explicit
configuration selection remain MS01 work; authored body execution remains MS07.
Generated artifact regeneration and the complete target matrix
remain integration/release work after the shared schemas stabilize.

## Next implementation work

Complete MS01 design-wide source/configuration handling. Implement MS02 resolved timing metadata, then
MS04 typed graph and the MS05 coordinator. The first circuit-wide slice must
include two HDL instances, an XSPICE participant, a loaded SPICE boundary,
off-grid timing and a rejected trial. Continue through every remaining milestone
in the approved plan; no parity claim is made at this intermediate checkpoint.
