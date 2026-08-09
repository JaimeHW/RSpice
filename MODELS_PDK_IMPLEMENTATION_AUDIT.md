# RSpice Models & PDK Implementation Audit

Audit date: 2026-07-31
Scope: the governed Models & PDK visual reference under
`mockups/rspice-workbench-host` (read-only) and the actual Rust/egui product
implementation in `crates/rspice-ui`.

## Executive verdict

No. The Models & PDK experience is not fully, thoroughly, or completely
implemented, not every mockup control is backed by a complete product
capability, and the area is not production- or commercial-release-ready.

The current implementation contains substantial real functionality:

- a six-page Models workspace;
- a governed project-owned Model Editor;
- a source-bound Model Correlation workflow;
- signed PDK package installation, validation, activation, rollback, and local
  trust administration;
- exact signed project PDK pins with checkpoint-bound, hash-chained change
  receipts;
- typed TT/SS/FF/SF/FS SPICE model-source contracts whose exact signed
  artifacts and package-relative include closures are materialized into the
  simulation input selected by the project pin;
- typed signed-package Verilog-A roots whose exact authenticated dependency
  closures are compiled, retained, transferred, and executed from a disjoint
  content-addressed virtual namespace with no host-filesystem fallback;
- schema-3 signed PDK callback contracts whose exact WebAssembly artifacts are
  ABI-validated at installation and can execute in a deterministic,
  capability-gated, fuel-metered, memory-bounded sandbox with no WASI or
  network imports and with content-digested execution receipts, plus an
  exact-project GUI workflow that supplies canonical active-plan variables and
  commits hash-chained, project-revision-bound execution evidence;
- an authoritative schema-1 physical-layout document domain with integral DBU
  coordinates, exact signed-project technology identity, revision-checked
  atomic transactions, nets, shapes, terminals, text, hierarchy, arrays,
  transforms, deterministic digests, project persistence, and cell/view
  lifecycle ownership;
- an initial governed Layout cellview editor that creates exact-signed-PDK
  documents, renders the active package-bound display profile, and supports
  rectangle insertion, selectable geometry, type-correct deletion, pan, zoom,
  fit, and phone-width accessibility semantics through the authoritative
  revision-checked transaction boundary;
- typed recognition and extraction package contracts; and
- fail-closed route and simulation/save gates.

Those are useful foundations. The release-target Library/Cellview specialist
workflow is now implemented and registered, but it has not been independently
qualified for commercial release. The implementation also does not close the
  preview specialist workflows, a production-complete scalable layout editor
  and stream I/O, layout recognition/extraction execution, platform and assistive-technology
qualification, foundry/domain validation, or commercial trust/entitlement
requirements.

This audit also corrected deterministic product defects in UI governance,
drawing-sheet/hardcopy workflows, project-library publication, signed PDK
model execution, default-stack session restoration, and result-workspace
integration. It also replaced the browser's single-record PDK package storage
with a transactional, content-addressed IndexedDB repository whose immutable
archives and canonical metadata are SHA-256 addressed, generation-bound, and
read back before live state is published. Browser publication now also
requests persistent origin storage, preflights the reported quota with a
conservative record-footprint estimate and retained safety reserve, reports
best-effort/unknown durability, and offers explicit in-product storage
recovery. The current full `rspice-ui` native library run executed 4,135 tests:
4,120 passed, 12 were intentionally ignored, and 3 failed. The failures are a
shortcut-ownership regression and two Report Authoring exposure/route
regressions; each also fails in isolation. All newly added callback,
physical-layout, Layout-view, and initial editor focused gates pass.
Current-state native and WebAssembly library checks pass, repository-wide
`cargo fmt --all -- --check` passes, targeted formatting passes,
`git diff --check` passes, and the mockup worktree is clean. Hundreds of
compiler warnings remain. An optimized WebAssembly artifact containing the
audited Models & PDK source was produced and launched in the in-app Chromium
browser under WebGPU. All six Models pages rendered and switched at desktop
width; PDK Technology Administration rendered at desktop width and at
emulated 820 x 1180 and 390 x 844 viewports. The phone run exposed a real
header overlap, which was corrected, regression-tested, rebuilt, and visually
rechecked with zero new console warnings, errors, or exceptions. This is one
Chromium environment with emulated viewports, not a supported-browser matrix
or real tablet/phone qualification. These repairs materially improve the
implementation, but they do not implement the missing product capabilities or
supply the external qualification evidence required for a commercial claim.

The mockup is not product code or an executable acceptance authority. Its
screens, responsive layouts, visible controls, states, and interaction intent
are used only to inventory and visually compare the implementation. Product
functionality and release readiness must be proved in Rust and against
supported runtime platforms.

## Audited surface inventory

| Surface | Mockup tier | Rust route | Current assessment |
|---|---|---:|---|
| Models primary workspace | Primary release workspace | Available | Substantial implementation; not independently qualified |
| Device Model Editor | Release target | Available | Real guarded editing and qualification workflow; not commercial-qualified |
| Library/Cellview/Symbol/Form authoring | Release target | Available | Real governed browser, symbol, terminal, parameter-form, and editor handoff workflow; commercial qualification and connected publication/collaboration remain open |
| PCell Designer | Preview | Unavailable | Design/mockup only |
| Model Extraction | Preview | Unavailable | Design/mockup only |
| Model Correlation | Preview | Available | Real source-bound workflow; preview, not release-qualified |
| Library Characterization | Preview | Unavailable | Design/mockup only |
| PDK Technology Administration | Preview | Available | Real signed-package administration; important boundaries remain |
| Protected Model/IP Execution | Preview | Unavailable | Design/mockup only |

The Rust route gate is explicit and fail-closed. It registers executors for
Model Editor, Library/Cellview Manager, Model Correlation, and PDK Technology
Administration. PCell Designer, Model Extraction, Library Characterization,
and Protected Model/IP Execution remain unavailable. This is correct product
behavior, but it is also direct evidence that the complete Models & PDK mockup
is not fully implemented.

## What is implemented

### Models primary workspace

The Rust workspace has the mockup-aligned tabs:

1. Models
2. Symbols & CDF
3. Corners & sections
4. Bins & geometry
5. Include graph
6. Qualification

Implemented behavior includes authenticated model-library imports, parsed
model/subcircuit catalog records, source provenance, conflict-aware definition
precedence, exact include closure, model sections/corners, model-bound symbol
contracts, engine-backed geometry-bin inspection, append-only bin audit
receipts, and source-bound qualification/correlation gates.

The UI does not fabricate an executable result when the engine cannot produce
one. During this audit, the complete-surface regression test exposed a stale
empty-state assumption: an authoritative inspection can validly succeed with
no geometry-binned executable families. The responsive layout assertion now
tracks that truthful empty receipt rather than requiring an inspection
failure. The Models and PDK focused suites and the complete current native
library suite pass.

### Project Library/Cellview foundation

The ordinary Project Library surface now exposes the existing real
library/cell/view state through a responsive three-column browser. Selection,
open, and where-used actions are backed by live workspace state. Its cell
actions now reach validated New View, Copy Cell, Rename Cell, Delete View, and
Delete Cell workflows instead of leaving those existing transactions
unreachable.

The entry points resolve the exact library/cell/view before opening, reject
stale and read-only targets without changing modal state, and expose only view
types that have production editors. Destructive actions use a dedicated
revision-bound confirmation that reports view count, open and dirty views,
loaded instance references, owned source bundles, configuration roots, and
project-root impact. Configuration-root targets cannot be confirmed.

New Cell, New View, Copy Cell, and Rename Cell also capture the exact library
catalog revision when their modal opens. An intervening catalog mutation
disables commit and requires the user to cancel and reopen the operation; the
commit handlers independently enforce the same optimistic-concurrency check
before validation or mutation.

Every successful create, copy, rename, or delete membership mutation now
advances exactly one project revision, marks project metadata dirty,
invalidates stale netlist/run evidence, and clears incompatible project-level
design history. Deletion continues through the existing source cleanup,
Design Management tombstone, buffer/tab/hierarchy pruning, and valid-focus
repair path.

Those mutations now also retain immutable, hash-chained receipts with the
exact operation identity and before/after project and library revisions.
The persisted library catalog has its own monotonic revision, while
presentation cleanup and serialization sanitization are explicitly
revision-neutral. An Audit History view exposes both membership and
publication receipts.

The runtime now accepts a trusted, content-digested snapshot from an external
lock authority. Library, cell, and view leases are hierarchical; wrong-project,
stale-revision, tampered, and intersecting lock snapshots fail closed. The
Project Library can inspect the exact authority, generation, revision,
digest, owner, scope, and lease timestamps. It does not self-assert an
organization lock authority or silently expire a remote lease.

A public two-phase publication boundary now prepares a content-addressed full
project artifact without mutating live state, requires the caller to durably
persist those exact bytes, and only then commits one immutable publication
receipt. Commit rejects intervening project or catalog revisions. Exact
rollback verifies artifact digest/size, project identity and revisions,
publication-ledger prefix, technology binding, and lock authority before
atomically restoring the published workspace and appending an actor-,
authority-, and reason-bound rollback receipt.

This closes important reachability and mutation-safety defects. It does not
provide a connected organization identity/permission service, remote lock
provider, remote publication object repository, or multi-user merge service.

### Library/Cellview specialist

The release-target `library-cellview-manager` now has a registered,
project-owned Rust executor under the Models workspace. Its persisted
Libraries and Symbol & form pages render responsively at desktop, tablet, and
phone widths. The Libraries page reuses the authoritative Project Library
three-column browser and therefore reaches the same validated, revision-bound
create/copy/rename/delete, lock-inspection, and audit operations rather than
maintaining a second synthetic catalog.

The Symbol & form page resolves the exact selected Library Manager symbol,
renders the authored `SymbolDocument` with the production Design renderer, and
shows the real typed model-bound symbol definition, terminal semantics,
parameter-form contract, generated views, source contract, revisions,
validation state, and executable netlist status. Editor, form, import, and
model-bound-symbol actions hand off the exact selected cellview to existing
production dialogs/editors. Legacy and invalid typed metadata fail closed and
cannot be presented as executable form or placement contracts.

Project-closed, safe-mode, and read-only-library states disable mutation with
an explicit reason while retaining read-only editor inspection where safe.
The route and page survive session serialization, retain Models workspace
ownership, expose accessible tab/table/action semantics, and are covered by
route, persistence, responsive-render, AccessKit, exact-selection,
safe-mode, and legacy-contract tests.

This closes the missing specialist-route implementation defect. It does not
provide connected organization principals, a shared remote lock service,
multi-user merge/conflict resolution, a remote publication object repository,
or formal assistive-technology/device qualification.

The Project Library and specialist browser now expose governed Publish and
Rollback transactions. Native publication uses the synchronized atomic writer.
Browser publication requires the File System Access API, writes the exact
prepared bytes, reads them back, and commits the immutable receipt only after
the digest matches. Download-only backends, cancellation, stale project or
catalog revisions, late destination changes, write/read-back errors, and
digest mismatches append no receipt. Rollback requires the exact retained
artifact, validates its receipt digest and size before enabling confirmation,
and reruns the full identity, revision, lineage, technology, lock, and audit
preflight before atomic restoration.

### Device Model Editor

The registered Model Editor supports a selected coherent project-owned model
card, typed parameters, section/statistical/temperature contracts,
parser-validated changes, retained revision comparison, executable
qualification vectors, guarded source/evidence commits, and fail-closed
promotion prerequisites. Built-in and external sources remain read-only.

This is a meaningful implementation, but unit tests and a route executor do
not establish model accuracy across foundry decks, numerical equivalence,
long-run stability, or commercial sign-off.

### Model Correlation

The registered correlation workspace retains immutable CSV datasets, exact
source and simulation provenance, unit-aware alignment, metrics and residuals,
append-only outlier dispositions, independent review evidence, and
qualification handoff. It correctly cannot promote a model or convert a failed
qualification vector into a pass.

It remains a mockup-classified preview workflow and lacks the external
measurement, domain, numerical, and platform evidence needed for a commercial
claim.

### Signed PDK administration

The PDK package domain now provides:

- exact Ed25519 manifest verification;
- manifest, archive, and artifact digest/size verification;
- bounded parsing and case-insensitive package-path uniqueness;
- layer/purpose and stream-map completeness checks;
- connectivity reference validation;
- callback count, artifact-size, ABI-version, entrypoint, import-signature,
  and capability validation, with network access forbidden;
- exact engine/viewer version and execution-target compatibility;
- immutable package revisions and exact active bindings;
- runtime-only validated catalogs after restart revalidation;
- hash-chained install, activate, rollback, trust-provision, and trust-revoke
  receipts;
- irreversible publisher-key revocation and immediate package quarantine;
- typed, source-bound recognition contracts with terminal/layer-purpose and
  qualification-vector closure; and
- typed, source-bound extraction contracts with covered quantities,
  layer-purpose scope, layout vectors, and reference artifacts;
- typed SPICE model-source roots for TT, SS, FF, SF, and FS processes, with
  required device domains and explicit artifact/section selections;
- installation-time validation that every model root references a signed
  Model artifact, every required domain is supplied, the TT contract exists,
  selected `.lib` sections materialize, dependency paths remain inside the
  package, and the exact dependency closure is executable and nonempty;
- content-addressed, host-path-free materialization under
  `/rspice-pdk/model-sources/{archive_digest}`, preserving package-relative
  `.include` dependencies and external `.lib file section` selections;
- exact project-pin-driven composition of signed PDK model cards with the
  ordinary sealed model-library bundle for generated runs and project-bound
  manual decks, while standalone manual decks remain self-contained;
- schema-2-or-newer signed Verilog-A source contracts with explicit root artifact,
  module, and netlist-alias selection; installation-time dependency-closure
  compilation; exact archive/artifact digest retention; generated and
  project-bound manual-deck directives; worker-transferable compiled runtimes;
  and fail-closed engine registration under `__rspice_pdk__/` with no ambient
  file fallback;
- schema-3 signed callback contracts with an exact versioned ABI, installation-
  time Wasm validation, capability-to-import closure, a fixed no-WASI host
  interface, a 10,000,000-fuel ceiling, an 8 MiB memory ceiling, strict
  instance/memory/table limits, and fail-closed traps for invalid guest memory
  or ignored host faults;
- exact-binding callback execution that revalidates the package archive and
  every artifact, exposes only signed project parameters and signed package
  bytes authorized by the declared capabilities, and returns deterministic
  metadata plus a tamper-evident receipt covering package, artifact, input,
  output, target, and fuel identity;
- prepared-run receipt identity for the signed PDK package/archive supplying
  the executable model cards and Verilog-A runtime;
- editable screen/print display profiles that cover every signed layer/purpose
  exactly once, including visibility, selectability, opacity, fill, outline,
  hidden-object, dimming, and selection presentation; and
- immutable package-digest-bound display-profile revisions with durable
  publish/activate/rollback receipts and fail-closed signed-default fallback
  when the active technology no longer matches. The initial governed Layout
  canvas now consumes visibility, selectability, screen color/fill, outline,
  and selection presentation, but pattern fidelity and the rest of the
  production renderer contract remain incomplete; and
- deterministic comparison of two currently trusted signed revisions with
  typed breaking, review-required, and informational changes across every
  manifest contract and exact archive identity.

The PDK administrator exposes package, layer, governed display-profile,
stream-map, connectivity, recognition, extraction, resource, and trust/audit
views, plus native and browser import flows. Display-profile actor and authority
fields are explicitly operator-provided local audit values; they do not claim
connected organization authentication. Only personal-device display profiles
can currently be published; project and organization scopes are visibly
unavailable and fail closed until their owning repositories and policy
services exist.

The four visible actions in the read-only PDK administration reference now map
to explicit Rust behavior:

| Visual-reference action | Rust owner and behavior | Status |
|---|---|---|
| Validate package | Signed import validation plus explicit revalidation of every retained archive against the current trust store | Implemented |
| Diff against revision | Deterministic comparison of two currently trusted signed revisions across identity, compatibility, layers, stream maps, connectivity, recognition, extraction, model-source contracts, callbacks, artifacts, manifest/archive digests, and breaking/review/informational impact | Implemented; current native regression gate passes |
| All layers | Full validated layer/purpose dictionary in the Layers view | Implemented |
| Display resources | Exact-package-bound immutable display-profile editor, activation, rollback, and signed-default fallback; the initial governed Layout canvas consumes core active-profile presentation, but the production renderer contract is incomplete | Partially implemented |

These checks and prepared-run tests prove that signed SPICE Model and Verilog-A
artifacts are selected by the exact project pin, revalidated, sealed without
host paths, and inserted into generated and project-bound manual simulation
inputs. The Verilog-A runtime is compiled from the authenticated package
closure, transferred as validated compiled state, bound to the exact signed
archive in the simulation snapshot, and installed at the core engine boundary
without host-file fallback. They do not prove foundry or numerical correctness
of those models. The PDK Administration Resources workflow can execute an
explicitly selected callback only for the project's exact signed package pin.
It supplies validated active-plan variables in canonical finite-SI form and
commits the input, sandbox receipt, plan identity, operator authority, reason,
project revision transition, and receipt-chain digest into the project-owned
Project Configuration document. The UI exposes verified receipt details and
exact JSON export with explicit accessibility semantics. Derived callback
metadata intentionally remains immutable evidence; no product engine consumes
it as a layout, model, recognition, extraction, PCell, or rule-deck mutation.
This path therefore does not execute rule decks, device recognition, parasitic
extraction, PCell generation, layout mutation, physical verification, or
sign-off.

### Authoritative physical-layout substrate

The workspace now owns versioned physical-layout documents rather than
representing layout as untyped metadata. A document is bound to one exact
Layout cellview and to the project's exact signed PDK package, revision,
manifest digest, archive digest, process, stack, and database unit. Coordinates
are stored as bounded integral DBU values. The domain validates rectangles,
simple polygons, paths, layer/purpose identity, nets, terminal/shape binding,
globally unique object identities, arrays, transforms, text, properties, and
hierarchical instance masters. Edits use expected-revision atomic
transactions: the candidate document is fully validated before publication,
and a failed edit leaves the authoritative document unchanged.

The documents persist in the project, participate in cellview and Project
Configuration checkpoint digests, overlay/revert and deletion, and are copied
or renamed through preflighted catalog transactions. Rename remaps every
hierarchical master reference. Project-file validation requires exact Layout
views, exact signed-PDK identity, authoritative master documents, and an
acyclic hierarchy. A project technology change that conflicts with existing
layout is rejected instead of silently reinterpreting DBU geometry.

An initial governed Layout cellview application now exists over that authority.
The New View workflow can create a Layout view only from the exact signed
project PDK, preflights the library, layout catalog, and project mutation before
publication, and leaves no partial state when authority is unavailable. The
canvas resolves the exact project package rather than the administrator's
active package, consumes the active immutable display profile with signed
defaults as a fail-closed fallback, renders rectangles, polygons, paths, text,
and hierarchical-instance placeholders, and supports rectangle creation,
selection, type-correct deletion, pan, anchored zoom, and fit. Every persisted
edit crosses the expected-revision application transaction boundary, and both
edit-time and project-authority validation reject shape or text layer/purpose
identities outside the exact signed project PDK without publishing partial
state. Toolbar, layer selector, actions, and canvas have explicit accessibility
semantics, including a focused phone-width render gate.

This is still an initial editor slice, not a production physical-layout
application. There is no spatial index or scalable persistent geometry store,
complete polygon/path/text/terminal/net/instance authoring, hierarchy descent,
selection sets, snapping/rulers/measurements, property inspection, physical
layout undo journal, exact display-pattern renderer and clipping/culling,
GDSII/OASIS importer or exporter, DRC/rule-deck executor, connectivity
extractor, device recognizer, parasitic extractor, PCell generator, LVS flow,
or foundry-qualified physical verification. Whole-document cloning currently
supplies transaction atomicity and is not a production performance design for
million-object cells.

### Project PDK binding

A project can pin an exact signed package identity, revision, manifest digest,
archive digest, publisher/key identity, process/stack identity, and supported
targets. Attachment or replacement is checkpoint-bound and records actor,
authority, reason, before/after binding, exact project revisions, and a
tamper-evident receipt chain.

Governed save and simulation validate the project descriptor, the exact
currently trusted installed package, and the authority receipt. A package
cannot silently mutate a pinned project. Simulation resolves model sources
from that exact project binding, never from the administrator's currently
active revision. Both generated project runs and receipt-backed manual project
decks carry the signed model cards and package identity into the prepared-run
receipt. A manual deck without project ownership does not inherit PDK state.

Attachment does not migrate schematic or physical design data between
technologies. A conflicting technology change is blocked once authoritative
physical layout exists; a future migration workflow must explicitly transform
and revalidate that data.

## Material gaps and release blockers

### P0: release-target qualification and connected governance incomplete

`library-cellview-manager` is explicitly a mockup release target. It now has a
registered Rust route executor backed by real Project Library, symbol,
terminal, parameter-form, import, editor, lock, and audit state. Its focused
implementation tests pass; a complete current native library rerun remains
pending.

That closes the previously missing route, but not the commercial qualification
contract. The release-target workflow still lacks:

- authenticated organization principals and per-operation permissions;
- a connected shared lock authority and multi-user merge/conflict resolution;
- a connected remote publication object repository (the local native and
  verified browser publication/rollback writers are implemented);
- specialist recovery/branching/version-browser workflows beyond the existing
  immutable mutation/publication receipts and project checkpoints; and
- accessibility and assistive-technology qualification for every mutation on
  supported desktop, browser, tablet, and phone targets.

These gaps still prevent release sign-off for the release-target workflow.

### P1: focused Models/PDK baseline is green, but the full suite is not

The focused display-profile slice passes all 7 tests, including immutable
publication/activation/rollback, adversarial integrity checks, persistence and
package revalidation, unavailable-scope fail-closed behavior, and a
phone-width AccessKit surface test. The signed technology-package slice passes
18 tests, the callback sandbox passes 6, signed revision comparison passes 7,
the PDK administrator passes 6,
and the prepared-run module passes all 34 tests, including signed SPICE and
Verilog-A materialization, project-bound manual decks, exact receipt identity,
and tamper/fail-closed cases. The complete 11-test core sealed Verilog-A runtime
batch passes, including signed-PDK execution without a host file and rejection
of ambient-file fallback.

The browser PDK persistence contract slice passes all 10 tests. It covers
metadata/archive separation, signed-registry round trips, tamper rejection,
exact path/generation head binding, canonical identity independent of map
insertion order, conservative record-footprint accounting, quota-reserve
admission, safe degradation when StorageManager estimates are unavailable, and
the existing native persistence contracts. The current complete 4,135-test
native library run produced 4,120 passes, 12 intentional ignores, and 3
deterministic failures. The failures are outside the audited Models/PDK and
layout slices, but they are current release blockers: the Next Violation
shortcut loses ownership after its first design activation, and two Report
Authoring tests disagree with the current command registry/route exposure.
Each failure reproduces in isolation. Native and WebAssembly library checks,
repository-wide formatting, targeted formatting, and diff integrity pass.
This is useful implementation evidence, not a commercial qualification
matrix. It does not include a frozen release commit, a supported-browser
matrix, real-device testing, independent accuracy
correlation, security review, or the external gates listed below. The compiler
still reports hundreds of warnings.

No mockup test result is used as product evidence. The mockup remains a
read-only visual/control reference.

### P0: no end-to-end control implementation proof

The mockup contains 1,253 unique design action contracts across 1,514
release-scope occurrences. Its generated traceability matrix proves that
design bindings exist, not that Rust handlers exist. For every audited Models &
PDK surface, the matrix says:

- `implementationApproved: false`;
- validation plan `design-plan-complete-execution-pending`;
- `executed: false`; and
- `qualificationSatisfied: false`.

A production claim requires a generated crosswalk from every visible reference
field/action to one Rust command/query owner, state transition, persistence
contract, undo/recovery behavior, authorization decision, accessibility
semantics, and automated test. No such complete implementation crosswalk is
present. The crosswalk is an implementation audit artifact; it does not turn
the mockup into product code or make its own test suite a release gate.

### P0: engineering engines and qualification are absent

Typed package declarations are not engineering execution. The current product
does not provide a qualified:

- layout device-recognition engine;
- parasitic R/C/coupling/L/device-parameter extraction engine;
- PCell geometry generator and migration engine;
- library characterization executor and publisher;
- protected-model execution/entitlement service; or
- foundry-approved sign-off flow.

Those capabilities must remain unavailable unless and until their producers,
reference vectors, accuracy tolerances, limitations, failure modes, and release
evidence are implemented.

### P0: signed model execution is not foundry-qualified PDK execution

The administrative trust boundary now continues into a real SPICE model
execution path: exact signed bytes are revalidated, dependency-closed,
materialized into content-addressed in-memory paths, selected by the project's
exact process contract, combined with sealed project model sources, inserted
into prepared simulation input, and identified in the run receipt. Installation
or administrator activation cannot silently redirect a pinned project.

That implementation closes the earlier end-to-end SPICE-artifact wiring
defect. The signed-package Verilog-A path now also validates typed source
contracts, compiles only the authenticated dependency closure, retains exact
archive and artifact identity, dispatches the runtime through prepared runs and
snapshots, and installs it in the engine's sealed virtual namespace without
ambient-file fallback. This is still not proof that either the cards or
Verilog-A models are numerically correct, complete, or accepted by a foundry.
There is no independent reference-correlation suite for each signed
process/domain/analysis combination. The bounded callback sandbox now has an
exact-project invocation workflow and durable, tamper-evident project receipts.
Its returned metadata is evidence-only: no qualified downstream product engine
applies it, and there is no rule-deck runtime. Recognition, extraction, PCell,
physical verification, and sign-off declarations remain metadata-only. These
remain direct blockers to a complete PDK or foundry-qualified claim.

### P0: trust and protected-IP boundary is not commercial-grade

The current trust store is durable local JSON with hash-chained audit records.
It is not an operating-system keychain/HSM, organization-managed trust service,
remote revocation service, or independently secured administrative authority.
A local attacker who can rewrite all local state is outside the integrity
guarantee.

Production licensing keys are also not configured, and Protected IP remains an
unavailable preview surface. Entitlement enforcement, target attestation,
secret isolation, redacted diagnostics, retention, revocation, and incident
response need their own security program.

### P0: platform and accessibility evidence is open

The current native Windows and WebAssembly non-test libraries compile. An
optimized browser artifact containing the audited Models & PDK source was also
launched in the in-app Chromium/WebGPU runtime. The six Models pages and PDK
Administration were exercised at desktop width; PDK Administration was also
rendered at emulated tablet and phone viewports. This does not constitute a
supported-browser or real-device qualification matrix. Linux cross-target
compilation passed in an earlier captured source state but is not current
release evidence.
The macOS cross-target check cannot complete on this Windows host because an
Apple-compatible C toolchain is absent; the `blake3` build script correctly
fails when the host `cc` cannot compile its macOS assembly. Android and iOS
Rust targets are not installed in the audited environment, so there is no
native mobile compile evidence. No real-device matrix, browser
lifecycle/storage matrix, screen-reader validation, touch/stylus study, or
mobile/tablet performance qualification has closed.

The browser PDK configuration now uses a transactional, content-addressed
IndexedDB repository. Signed archives are immutable SHA-256-addressed objects;
canonical metadata and a small generation head are stored separately. Writes
compare the exact predecessor head, read back the object/head/full snapshot,
and publish the candidate to live application state only after verified
commit. A legacy `localStorage` record is migrated and removed only after that
commit. Restored packages receive no inherited runtime trust and are
revalidated against the current publisher trust store.

That removes the deterministic single-record size blocker, but it is not a
complete browser qualification result. In the one interactive Chromium run,
`navigator.storage.persisted()` returned false, the application truthfully
reported best-effort durability and approximately 10.0 GiB available, and
read-only inspection found the expected `rspice-pdk-config` version-1
IndexedDB database and `content-addressed-pdk-config` store. The empty registry
correctly had no head or archive records. No signed browser package fixture was
imported during this run, so a real-browser write/restore/rollback lifecycle
was not claimed. Per-origin quota exhaustion and eviction behavior,
private-mode behavior, persistent-storage decisions across browsers,
transaction interruption, lifecycle recovery, and realistic maximum-package
tests remain unqualified on supported browsers and devices. Storage failures
remain fail-closed.

The mockup declares six unsatisfied external gates:

1. human usability validation;
2. assistive-technology validation;
3. real-device/platform validation;
4. domain-SME review;
5. independent implementation ambiguity review; and
6. final implementation sign-off.

### P1: implementation hygiene is not release clean

The current native library check emits 372 warnings and the current
WebAssembly check emits 344 warnings. The last completed Linux cross-target
check emitted 546 warnings, but it is not evidence for the current source
state. Most observed warnings are unused imports and dead/unreachable product
surfaces. Repository-wide formatting now passes. A commercial release
candidate should still have an explicit zero-warning policy or a reviewed,
owned exception ledger. Silencing these warning classes globally would hide
unfinished surface ownership rather than fix it.

The workspace is also extensively dirty, including untracked implementation
files. That is acceptable for active development but not a reproducible
release candidate. A release audit needs a frozen commit, clean dependency
inventory, SBOM/license/security results, signed artifacts, and repeatable CI.

### P1: visual fidelity and downstream interaction remain incomplete

The Rust administrator follows the reference's information hierarchy and
actions, but it is not a complete visual implementation of the PDK reference.
The reference makes the process stack the primary package identity with a
technology cross-section diagram and uses a dense layer-swatch composition.
The Rust surface currently presents a conventional header, metrics, tabs,
property panels, and layer table; it has no technology cross-section
visualization. The governed display-profile editor is materially richer than
the reference control, and its active state now affects the initial governed
Layout canvas, but that canvas is not a production-complete renderer. The
limited Chromium run did catch and close one phone-width header
overlap, but it does not replace automated visual regression, touch-target,
keyboard-flow, truncation, contrast, supported-browser, real-device, or real
assistive-technology inspection.

### P1: remaining PDK product gaps

- No via/contact generator or PCell execution.
- An authoritative physical-layout document/persistence/lifecycle substrate
  and an initial governed Layout editor now exist, but there is no
  production-complete scalable editor/renderer, spatial database, GDSII/OASIS
  I/O, or physical-layout undo journal.
- The initial canvas consumes core governed display-profile state, but it does
  not yet provide exact fill-pattern fidelity, production clipping/culling,
  large-cell performance, or the full hidden/dimmed presentation contract.
- The content-addressed browser artifact repository lacks cross-browser quota
  exhaustion, eviction, private-mode, interruption/recovery, signed
  write/restore/rollback, and maximum-package qualification across supported
  desktop and mobile browsers.
- No foundry/reference-correlation qualification proving the integrated signed
  SPICE cards and Verilog-A models across every declared process, domain,
  analysis, and platform.
- Signed callback output is retained as exact project evidence, but no
  qualified downstream workflow consumes the derived metadata; no rule-deck
  runtime exists.
- No layout-recognition execution.
- No extraction execution or reference correlation.
- No foundry acceptance or sign-off qualification.
- No technology-migration workflow when changing a project binding.

## Verification performed

| Verification | Result |
|---|---:|
| Model library state tests | 133 passed |
| Models surface tests | 21 passed |
| Model Editor tests | 13 passed |
| Model Correlation tests | 10 passed |
| PDK technology-package tests, including signed Verilog-A closure and adversarial validation | 18 passed |
| Signed PDK callback sandbox, including capability, exact-package-byte, resource-limit, downgrade, and guest-memory adversarial cases | 6 passed |
| Exact-project callback workflow, including canonical plan input, two-entry receipt chaining, project dirty-state ownership, missing-pin transaction rollback, responsive AccessKit receipt controls, and selective Project Configuration overlay/revert ownership | 4 focused tests passed |
| Authoritative physical-layout domain and product integration, including atomic edits, geometry/net/terminal invariants, tamper rejection, deterministic digests, exact signed-PDK/DBU initialization, signed layer/purpose enforcement, imported foreign-layer authority rejection, cell copy/rename hierarchy remapping, missing-master rejection, and recursive-hierarchy rejection | 14 focused tests passed |
| Initial governed Layout editor geometry and phone-width accessibility slice | 4 passed |
| PDK display-profile domain, persistence, and responsive accessibility slice | 7 passed |
| PDK signed-revision diff domain tests, including callback ABI contract visibility | 7 passed |
| PDK administrator surface tests | 6 passed |
| PDK persistence contracts, including content-addressed browser snapshots and quota admission | 10 passed |
| Prepared-run module, including signed SPICE/Verilog-A PDK execution and receipts | 34 passed |
| Simulation snapshot module, including signed-PDK Verilog-A runtime/archive provenance | 31 passed |
| Core sealed Verilog-A runtime batch, including signed-PDK execution and ambient-file fail-closed behavior | 11 passed |
| Project technology receipt adversarial test | passed |
| Run-readiness technology gate tests | 6 passed |
| Check/save technology validation tests | 3 passed |
| New Cell lifecycle tests | 4 passed |
| New View lifecycle tests, including exact-PDK Layout creation and transactional missing-PDK rejection | 11 passed |
| Cell operation entry/commit tests | 4 passed |
| New Cell command revision-capture test | passed |
| Project Library surface tests, including publication/rollback and lock UI | 13 passed |
| Library/Cellview specialist surface tests | 7 passed |
| Library/Cellview route, ownership, availability, and persistence aggregate | 11 passed |
| End-user publication/rollback transaction tests | 6 passed |
| Library transaction dialog aggregate | 33 passed |
| Project library lock-authority tests | 2 passed |
| Project workspace domain tests, including mutation receipt/revision tests | 67 passed |
| Project lifecycle tests, including exact save/snapshot revision tests | 25 passed |
| Project-file I/O tests, including missing authoritative layout-master and recursive-layout-hierarchy rejection | 76 passed |
| Cell/view deletion lifecycle tests | 16 passed |
| Copy/rename lifecycle tests | 5 passed |
| Library deletion modal-ownership test | passed |
| Current complete `rspice-ui` library suite | 4,120 passed, 3 failed, 12 intentionally ignored; 4,135 total; all 3 failures reproduce in isolation and are outside the focused Models/PDK/layout slices |
| Ignored mockup-parity subset | 10 passed |
| Ignored license-key minting commands | 2 operator-only commands; intentionally excluded from automation |
| Canonical ngspice corpus | 113 passed, 0 failed, 0 skipped |
| GF180MCU device corpus | 828/828 cases within contract; worst deviation 0.523% |
| Full serialized workspace run | incomplete: exceeded the 30-minute harness window before an owned final exit code |
| Workspace all-target compile (`--no-run`) | previously passed; not rerun as current release evidence |
| Native library `cargo check -p rspice-ui --lib` | passed, 372 warnings |
| WebAssembly library `cargo check` | passed, 344 warnings |
| Linux cross-target `cargo check` | previously passed with 546 warnings; current-state rerun pending a compilable workspace |
| macOS cross-target `cargo check` | not qualified; host lacks an Apple-compatible C toolchain |
| Android/iOS compile and device qualification | not executed; targets/devices absent |
| `cargo fmt --all -- --check` | passed |
| Targeted Models, PDK, Results-integration, and lifecycle `rustfmt --check` | passed |
| `git diff --check` | passed; existing LF-to-CRLF worktree notices remain |
| Rust/egui interactive visual QA | optimized artifact containing the audited Models & PDK source launched under in-app Chromium/WebGPU; all six Models pages exercised at desktop width; PDK Administration exercised at desktop width and emulated 820 x 1180 and 390 x 844; phone header overlap found, fixed, rebuilt, and rechecked; zero new console warnings/errors/exceptions |
| Browser PDK runtime storage inspection | Chromium reported `persisted=false`, quota 10,737,420,223 bytes, usage 1,983 bytes, and the expected version-1 IndexedDB database/store; empty-registry startup only, not a signed write/restore lifecycle |
| Mockup visual/control inspection | read-only reference only; no mockup test result accepted as product evidence |
| Rust dependency advisory/license/source policy (`cargo deny check`) | passed; duplicate-version warnings remain |

Passing these tests means the exercised contracts behave as specified. It does
not establish physical-model correctness, simulator precision, foundry
acceptance, security certification, usability, accessibility, device support,
or equivalence to Cadence Spectre, Virtuoso, or Keysight ADS.

## Production-readiness decision

**Decision: reject release / reject sign-off.**

The correct current product posture is:

- keep unavailable specialist routes fail-closed;
- label Model Correlation and PDK Administration as preview;
- do not claim recognition, extraction, PCell, characterization, protected-IP,
  foundry, or sign-off capability from package metadata;
- do not claim mobile/tablet qualification from a WebAssembly compile; and
- do not claim implementation completeness from the mockup's design-contract
  test suite.

## Required closure sequence

1. Qualify the implemented release-target Library/Cellview manager and connect
   its organization identity/permission, shared-lock, remote publication
   repository, and collaboration boundaries.
2. Generate an exact mockup-control-to-Rust implementation crosswalk and close
   every missing or intentionally omitted control.
3. Complete the production physical-layout application over the authoritative
   document substrate and initial governed editor: scalable spatial
   storage/indexing, complete geometry/hierarchy/connectivity authoring, exact
   display-profile rendering, stream import/export, undo/recovery, and
   platform-qualified interaction.
4. Decide which preview surfaces ship; implement each selected producer
   end-to-end or keep it unavailable. Signed SPICE and Verilog-A artifact
   materialization and execution now exist, and signed callbacks have an owned
   exact-project invocation/evidence workflow. Next closure requires foundry
   reference correlation, an explicit supported meaning and qualified consumer
   for callback-derived metadata (if any), and an explicit support decision
   plus implementation for rule-deck artifacts.
5. Establish a production trust, signing, entitlement, and protected-IP
   architecture with independent security review.
6. Qualify the implemented content-addressed browser repository across every
   supported browser/device, including quota and eviction policy, persistent-
   storage behavior, realistic package-size limits, interrupted transactions,
   retry UX, and recovery evidence.
7. Add reference-correlation suites for models, corners, bins, recognition,
   extraction, and supported simulators/platforms.
8. Qualify desktop, browser, tablet, and phone on real supported devices,
   including assistive technology and lifecycle/recovery behavior.
9. Freeze a clean release candidate, eliminate or formally disposition
   warnings, run the full CI/security/license/performance matrix, and retain
   signed evidence.
10. Execute and close all six external handoff gates against that exact frozen
   candidate.

Until all applicable items close, the Models & PDK workspace is an advanced
development implementation with strong fail-closed foundations, not a complete
commercial production release.
