# Netlist, Verilog-A, and Automation workspace implementation audit

Status: active; implementation substantially complete, release qualification open
Updated: 2026-08-09
Authority: `NETLIST_SCRIPT_EDITOR_IMPLEMENTATION_PROMPT.md` and
`mockups/rspice-workbench-host/implementation/netlist-script-editor-contract.json`

## Decision

The mockup is a visual reference, not a file-format or sample-data
specification. Names such as `characterize.py`, `runplan.rspice.yaml`,
`requirements.lock`, and `permissions.toml` are examples only. RSpice now
models arbitrary valid project paths, stable document identities, explicit
roles, ownership, revisions, capabilities, and dependency closure.

The scoped product systems are implemented, including the managed Python
runtime boundary, Netlist document system, multi-file Verilog-A workspace,
shared source editor, file lifecycle, diagnostics, recovery, and responsive
desktop/browser/tablet surfaces. They are not yet release-certified. A fresh
integrated UI test binary was built and the complete focused workspace matrix
passes. The same binary passes every repository UI test except one explicitly
filtered PSpice compatibility qualifier. Locked, warnings-denied native and
WASM release-library checks pass. A workspace-current optimized browser module
was engine-validated and packaged twice into byte-identical authenticated
archives. Exact packaged-browser interaction and native assistive-technology
campaigns remain open because the in-app browser controller fails before
launching the page and the shared tree still contains an explicitly temporary
Results visual-QA harness. Two unrelated Results-viewer files changed after the
final module and test binary; no scoped workspace or browser asset changed.

No customer or external-team setup is part of this design. In particular,
RSpice never discovers or executes Python from `PATH` and never asks the user to
install Python.

Phone-specific UI remains excluded by direction. This audit covers desktop,
browser, and tablet-like layouts.

## Required Python architecture

| Platform | Product runtime | Authority and distribution |
|---|---|---|
| Native desktop | CPython 3.14 | Signed, content-addressed, app-local runtime staged by the native release; exact ABI/API/platform/environment identity is authenticated before launch. System Python is neither detected nor used. |
| Browser | Pyodide 314.0.2 | Self-hosted closed asset inventory in a dedicated worker; runtime digest, bootstrap, packages, cancellation, and the qualified 2 GiB WebAssembly memory ceiling are enforced by the product. |

The Python entry file, run-plan document, lock document, permission manifest,
and helper modules are selected by persisted project roles. Their presentation
names and paths are not reserved.

## Contract traceability

Status vocabulary:

- `implemented`: production behavior exists and has focused automated evidence.
- `qualification-open`: production behavior exists, but a required artifact or
  platform campaign has not completed against the current shared tree.
- `moving-tree-open`: the scoped workspace and the production release
  libraries passed, but later concurrent product edits currently prevent an
  immutable whole-tree artifact from being qualified.

| Contract area | Status | Current implementation evidence |
|---|---|---|
| Scope and document roles | implemented | Stable project/bundle/document IDs, arbitrary logical paths, explicit language/role/owner bindings, immutable external ownership, and revision/digest identity. Demo filenames and values are not special cases. |
| Exact command context | implemented | One fail-closed context resolves project, document, kind, ownership, revision, capability, page, and platform. Menus, shortcuts, palette actions, save, compile, validation, search, and execution use that authority. Cell-view Verilog-A and singleton Code workspaces remain distinct. |
| Shared editor | implemented | Undo/redo, transactional IME, clipboard, multi-cursor, column selection, line operations, comments, indentation, folding, bracket/navigation commands, formatting routes, and two-axis virtualized rope editing. Read-only/governed documents reject direct and delayed mutation. |
| Text fidelity | qualification-open | Netlist retains encoding, BOM, and line endings; code boundaries reject invalid UTF-8; million-character horizontal virtualization and physical invalid-byte offsets have focused evidence. Repeat the interaction matrix in the final packaged browser artifact. |
| Language workflows | implemented | Completion, hover, signature help, declaration/definition, references, symbols, conservative rename, quick fixes/code actions, and formatting. Project rename is atomic and refuses any governed or external target. Managed CPython/Pyodide is the only Python grammar/evaluation authority. |
| Search and replace | implemented | Selection/document/open-document/language-project scopes; case/symbol/regex/comment/generated options; 500 displayed and 50,000 streamed limits; stale/cancelled/closed/permission states; atomic replacement only in eligible project-owned sources. |
| Canonical diagnostics | implemented | Stable canonical identity, source/range, related locations, quick fixes, suppression, revision, validation ID, currentness, and consumer set. Editor, Problems, Inspector, status, and validation views project one bounded collection rather than maintaining independent counts. |
| Netlist document system | implemented | Immutable generated deck and editable owned root/include models; source maps; dependency closure; validation and execution receipts; revision history; comparison/restore; exact generated/owned authority; canonical execution closure. |
| Netlist import and qualification | implemented | Transactional raw-byte staging, encoding/EOL/dialect evidence, include/parser checks, explicit transformation review, versioned execution profile and receipt, blocking unsupported/ignored/defaulted semantics, legacy quarantine, and in-product requalification. No silent translation. |
| Netlist archive | implemented | Deterministic ZIP import/export with schema, safe paths, declarations, CRC-32, SHA-256, source-map and dependency validation, size ceilings, and no filesystem fallback. |
| Netlist semantic boundary | implemented for workspace scope | Authored bytes and qualification identity are preserved through dispatch and unsupported semantics fail closed with actionable diagnostics. Adding simulator-engine vendor devices/statements or PSpice numerical compatibility is a simulator-core objective, not a reason to weaken or defer this workspace. |
| Verilog-A workspace | implemented | Multi-file bundles; include paths, definitions and compile order; IEEE-oriented diagnostics; virtual closure; stale compile rejection; source-bound receipts; native/WASM target reports; arbitrary-name root import; empty-workspace create/import recovery. |
| Verilog-A elaboration and specialist qualification | implemented | Versioned TOML build profile, package/version identity, exact entry selection, cell bindings, hierarchy/discipline checks, structural elaboration shared by backends, hidden-state/discontinuity/unit/convergence/portability checks, immutable qualification attempts, and blocking-error receipt refusal. |
| Managed Automation workspace | implemented | Generalized role-bound Python/run-plan/lock/permission closure, deterministic plan/evidence/artifact generation, validation receipt, governed dispatch snapshot, exact baseline/tolerance correlation, cancellation, stale-token rejection, and no ambient authority. |
| Automation execution and debugger | implemented | Authenticated native/browser worker protocol, structured host calls, breakpoints, continue/pause/step/restart/stop, stack/frames/locals/globals/watches/exceptions, structured output, worker recovery, resource limits, capability enforcement, and secret/ambient-operation failure closure. |
| File lifecycle | implemented | New/open/import/drop/save/save-as/rename/move/duplicate/delete/revert/history/compare; owned dependency copy and external relink; native compare-and-exchange save; external-change review; two-/three-way merge; dirty-close/save-all; late picker and permission/read-only reauthorization. |
| Crash recovery and project switch | implemented | Integrity-bound native/browser checkpoints close over Netlist, Verilog-A, Automation, helper and role documents. Browser cataloging streams one project-sized record at a time. Project replacement clears stale documents, diagnostics, jobs, search, debugger, results, and console authority. |
| Scale | qualification-open | Implemented boundaries: 10,000 files, 5,000,000 lines/400 visible rows, 1,000,000 diagnostics, 50,000 search results, 10,000,000 Automation tasks, 5,000,000 structured-log records, and 100,000 artifact records. Focused optimized tests exist; repeat peak-process-memory and cancellation observation against the final browser package. |
| Accessibility | qualification-open | Keyboard routes, named AccessKit actions, bounded active-line/diagnostic context, focus rings/restoration, contrast policy, reduced motion, responsive/coarse-pointer targets, 400% reflow policy, and empty-state recovery semantics are implemented. Final native screen-reader/focus and real-device coarse-pointer campaigns remain. |
| Localization and visual fidelity | qualification-open | Typed localization catalog, named interpolation, pseudolocale validation, literal-widget ratchets, shared design-system metrics, and desktop/tablet/narrow hierarchy are implemented. Final packaged screenshots and expanded-locale accessibility captures remain. |
| Release/security/resilience | moving-tree-open | Workspace source is formatting/whitespace clean; native/WASM release libraries, managed-runtime packages, browser/static gates, optimized WASM binding, deterministic packaging, WebAssembly validation, and the current integrated focused/aggregate test boundaries pass. The temporary visual-QA harness, dirty-tree source provenance, and unavailable browser controller keep immutable publishable and dynamic qualification open. No stale artifact hash is accepted as publishable evidence. |

## Fail-closed hardening completed in this pass

The interrupted integration had reintroduced production panic shortcuts. They
were removed from the complete scoped production surface:

- Automation lexical cursor and stage-construction invariants now produce
  `AUT018` and stop safely instead of unwinding.
- Managed debugger stops without a session ID, missing manifests, vanished
  plans, capability serialization, capacity diagnostics, and missing prepared
  receipts return explicit failures.
- Verilog-A worker/capacity fallback publication no longer assumes diagnostic
  construction can never fail.
- Netlist revision comparison, comparison close, and owned-source restore no
  longer rely on guarded `unwrap` calls.
- Structured Automation log chunk allocation reports a storage error instead
  of panicking.
- Source-level ratchets now reject `expect`, `unwrap`, `panic!`, `unreachable!`,
  `todo!`, and `unimplemented!` in these production transition modules.

## Current reproducible evidence

The following results were produced from the current workspace on 2026-08-09:

| Gate | Result |
|---|---|
| Locked/offline Cargo metadata | pass |
| Managed Automation protocol/runtime/package/qualifier Rust tests | 18/18 pass |
| Managed Automation protocol/runtime warnings-denied all-target Clippy | pass |
| Browser/runtime/IDE-worker/WASM-playground/web-packager static group | 31/31 pass |
| Native managed-Python staging and deterministic native packager tests | 5/5 pass |
| Independent warnings-denied Verilog-A compiler library | 173/173 pass |
| Independent Verilog-A compiler `wasm32-unknown-unknown` warnings-denied check | pass |
| Aggregate `rspice-ui` native warnings-denied release-library check | pass against the workspace-current source snapshot; later changes are confined to unrelated Results-viewer files |
| Aggregate `rspice-ui` `wasm32-unknown-unknown` warnings-denied release-library check | pass against the same workspace-current source snapshot |
| Current Automation parser isolated strict qualifier | 3/3 pass: canonical Unicode compile, malformed multibyte diagnostic, production no-panic ratchet |
| Current managed-Python bootstrap contract | 1/1 pass; exact Python entry, run plan, lockfile, permission manifest, dependency closure, and semantic role bindings |
| Current integrated Code workspace suite | 91/91 pass, including release-scale and production no-panic ratchets |
| Current integrated Netlist state suite | 40/40 pass |
| Current integrated Netlist document suite | 53/53 pass |
| Current Netlist/Verilog-A/Automation surface suite | 12/12 pass |
| Current checkpoint/recovery suite | 5/5 pass |
| Current browser file-import suite | 4/4 pass |
| Current managed-runtime application binding | 2/2 pass |
| Current aggregate integrated UI boundary | 4,469 passed, 0 failed, 14 ignored, 1 exact test filtered out of 4,484. The only filtered test is `qualified_pspice_import_persists_exact_versioned_profile`, explicitly outside this objective; its independent run fails closed on a PSpice behavioral-source expression. |
| Scoped production placeholder/panic scan after hardening | zero findings |
| Scoped Rust formatting and `git diff --check` | pass; line-ending conversion warnings only |
| Optimized browser snapshot | raw WASM 110,979,451 bytes, SHA-256 `34BC2841D6180D6D00954CE0253DED524721E9E87ABA21A5297C985D571699C8`; bound WASM 104,668,620 bytes, SHA-256 `D109F8C08FBA59FDE3B60C65D585D5576DDBC754286A8EC125DEA756744DD378`; JS 173,820 bytes, SHA-256 `1FD21E324B619A3D5306F69F670AC65DF1C5B755D5AE2BB77E01964FF1869543` |
| WebAssembly engine validation | pass; 41 exports and all three required worker exports present |
| Deterministic browser package | two byte-identical 32,904,191-byte archives, SHA-256 `2FA75D519C32BD3A4A38206BF4DEE81231F66C65174C75E93A9F912EB1BD6C19`; byte-identical 87-byte sidecars, SHA-256 `F582ED247DD9E47454365A253CFB5D45E6B983D74791C39F17995A9CA9B7E5FB`; 16 entries, one timestamp, 15 authenticated payloads, asset digest `053f98903b600b67a5122fe8bf9d9294f73de08198533b48dc611887c45d0389` |
| Exact package local HTTP retrieval | pass: entrypoint, bound JS, and Automation worker return HTTP 200 from the authenticated asset root |

The final workspace-current integrated binary contains 4,484 registered tests. Its complete
focused workspace matrix is green. A whole-binary run excluding only the exact
PSpice compatibility test passes with zero failures. The final optimized
browser module is newer than every relevant production source file at its build
boundary. Later changes are confined to unrelated Results-viewer files. The
module remains qualification evidence rather than a publishable release because
the manifest commit does not identify the dirty worktree and temporary debug
harness as an immutable source revision.

## Current qualification blockers

An explicitly temporary `RSPICE_DEBUG_OPEN` Results visual-QA harness remains
in `workbench/app.rs` and `workbench/app_state.rs`; the source says it must be
reverted before commit. The current test binary compiles because the hook's
test symbol was corrected, and all focused workspace tests pass. A normal
debug-library check still fails because the temporary helper is compiled under
`debug_assertions` while its byte-loader dependency is test/WASM-only. Native
and WASM release-library checks pass because this harness is correctly absent
from customer builds. The hook was not modified by this objective.

Concurrent schematic/XSpice and Results work that initially failed the
aggregate suite was subsequently corrected by its owners. Every one of those
exact tests now passes. One PSpice import-profile test remains red because its
fixture contains a behavioral source that canonical parsing rejects. PSpice
compatibility is explicitly outside this objective and no compatibility work
was performed here. The aggregate qualified-scope run filters only that exact
test and passes every other test.

The deterministic browser archive is built from the workspace-current production
source and authenticated by its asset digest, but its release manifest identifies only
the unchanged Git commit while the implementation is a dirty shared worktree.
It must not be published until the source is committed as an immutable revision
and the explicitly temporary visual-QA harness is removed. Two unrelated
Results-viewer files changed after the final workspace artifact; therefore the
archive is current for this objective but not the latest whole-application
worktree snapshot.

The in-app browser controller also currently fails before RSpice starts while
creating its own kernel assets (`os error 3`). Static worker/runtime/package
qualification remains valid, but it is not a substitute for the final dynamic
packaged-browser campaign.

## Remaining qualification order

1. Remove the temporary Results visual-QA harness and record the implementation
   as one immutable source revision; then rebuild the complete UI suite and
   release artifacts from that exact revision.
2. Reproduce the optimized browser module and deterministic package from the
   immutable final source. Do not publish the qualified dirty-tree snapshot.
3. Run packaged-browser editing, IME, clipboard, file lifecycle, worker
   cancellation, 2 GiB peak-memory, durable artifact, checkpoint, recovery,
   400% reflow, expanded-locale, and tablet/coarse-pointer campaigns.
4. Run native keyboard/focus/screen-reader, reduced-motion, high-contrast,
   crash-recovery, permission-revocation, and package-tamper campaigns.
5. Record only artifact-bound, reproducible evidence here; no mockup sample
   value or stale dirty-tree package may satisfy a release gate.

All remaining work is product-repository implementation or qualification. It
does not require customer-installed Python, manual source conversion, an
external compatibility team, or any other external deliverable.
