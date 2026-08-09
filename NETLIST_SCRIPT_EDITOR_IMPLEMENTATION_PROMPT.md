# RSpice Netlist & Script Editor implementation contract

You are responsible for fully implementing RSpice's Netlist & Script Editor workspace.

## Repository

`C:\Users\James\Desktop\RSpice`

The repository can contain concurrent work. Preserve every unrelated change. Never reset,
revert, overwrite, stage, or commit files outside this scope. Use a separate worktree or target
directory when it is needed to avoid contending for Cargo build artifacts. Commit only coherent
changes owned by this implementation effort.

## Primary objective

Replace any incomplete or legacy Netlist & Script Editor GUI with the latest approved design in
the current mockup, then make every represented feature fully functional, integrated, tested,
and production-ready. This is commercial engineering software. The implementation must be a
credible foundation for Spectre/ADS/Virtuoso-class netlist, behavioral-model, and automation
workflows; visual similarity alone is not completion.

The delivery is self-contained. Do not defer required product behavior, repository changes,
fixtures, build automation, tests, qualification, or implementation decisions to an outside team,
specialist, service, or future manual follow-up. Optional independent review may add confidence but
is never a completion prerequisite. Release secrets remain protected build-authority inputs rather
than product implementation work, and end users must not install or configure Python.

## Authoritative design and product contracts

The visual and interaction authority is:

`C:\Users\James\Desktop\RSpice\mockups`

The qualified Netlist & Script Editor implementation contracts are:

- `mockups/rspice-workbench-host/implementation/NETLIST_SCRIPT_EDITOR_IMPLEMENTATION_READINESS_PLAN.md`
- `mockups/rspice-workbench-host/implementation/netlist-script-editor-contract.json`
- the shared design-system, command/action, accessibility, large-data, and handoff contracts
  referenced by those files

The mockup is a visual and behavioral reference, not a literal data fixture or a substitute for
the product implementation. Example project names, file names, paths, source text, Python
versions, API versions, analyses, corners, permissions, lock data, diagnostics, results, and
artifact names demonstrate states and workflows only. Do not hardcode them as required product
values. Implement generalized systems that can create, open, edit, validate, persist, compile,
execute, debug, recover, search, compare, and navigate arbitrary supported documents.

Re-audit the current repository and module organization before editing. Do not rely on remembered
paths, an old mockup, cached browser content, or a superseded UI architecture. Locate the newest
Netlist & Script Editor sources and the currently served build.

## Strict scope

- Work on the Netlist & Script Editor workspace and directly supporting domain/application code.
- The scope includes the generated/project-owned netlist editor, source comparison, import and
  normalization workflows, Verilog-A multi-file editing/build workflows, Python automation and
  run-plan workflows, the shared code-editor substrate required by those surfaces, and the exact
  navigation, persistence, diagnostics, and execution handoffs they require.
- Do not redesign Project Overview, Schematic, Results, Simulation, Models/PDKs, or Verification.
  Cross-workspace changes are permitted only when essential to a specified Netlist workflow, such
  as opening a source owner, dispatching a validated run, or opening an immutable result/report.
- Results and reports remain owned by the Results workspace. The code workspace may request them
  or navigate to them but must not create a second report-authoring system.
- Desktop native and desktop browser are required. Narrow desktop and tablet-like layouts are
  required. Phone/mobile-specific design is deferred.
- Replace incomplete legacy shell behavior with a clean implementation aligned with the current
  refactored EGUI architecture. Reuse a shared component only when its semantics, quality, and
  design fit exactly.

## Mockup fidelity

- Match the latest mockup closely: hierarchy, proportions, panel widths, row heights, typography,
  colors, borders, spacing, padding, alignment, tabs, toolbars, status areas, trees, editors,
  gutters, minimaps, tables, forms, empty states, selected states, hover/focus states, menus,
  dialogs, overlays, scrolling, splitters, and resizing.
- Do not invent new persistent GUI elements, pages, dialogs, tabs, controls, or status surfaces
  that are not specified by the mockup or an accepted implementation contract.
- Commercial high-end simulator parity is the functional target, while the qualified mockup stays
  the design authority.
- Before removing existing product capabilities that are absent from the mockup, determine whether
  they are obsolete, duplicated, or still required. Ask before removing a non-obsolete capability.
- Ask before adding a new surface when a necessary workflow is genuinely ambiguous or absent.
- Do not expose internal IDs, debug receipts, closure hashes, capability plumbing, provenance, or
  duplicate status indicators unless the design explicitly calls for them. Keep required evidence
  in the domain/application model and show it only through an approved inspector or diagnostic.
- Empty space must be intentional. Prevent clipping, accidental borders, unstable form geometry,
  overlapping text, oversized blank regions, unbounded growth, and layout shifts.
- Use the established icon system and bundled fonts; do not depend on Unicode symbols that can
  render as missing glyphs.

## Mockup synchronization

- The mockup is the durable design reference; do not casually redesign it during implementation.
- If the user requests a design change, update both canonical mockup sources and the product so a
  later implementation pass cannot restore a superseded design.
- If implementation reveals a necessary design change, explain it and obtain approval before
  changing the mockup.
- Keep canonical mockup sources and their served/public copy synchronized with the repository's
  existing scripts.

## Required audit before implementation changes

1. Read all repository instructions.
2. Confirm the active branch and inspect the dirty worktree for concurrent changes.
3. Locate the latest canonical and served Netlist & Script Editor mockup sources.
4. Locate the refactored Rust/EGUI implementation and relevant parser, compiler, simulation,
   project persistence, Python, worker, and result-navigation services.
5. Inventory every Netlist, Verilog-A, and Automation surface, state, action, menu, dialog, tree,
   editor, toolbar, context menu, empty/error/loading/read-only state, and navigation path.
6. Inventory the common IDE behavior: commands, shortcuts, document lifecycle, search/replace,
   language services, diagnostics, debugger, accessibility, scale, recovery, and platform states.
7. Compare the complete inventory with the current implementation and the machine-readable
   contract.
8. Identify existing product functionality not represented by the mockup before removing it.
9. Record gaps by stable requirement or audit identifier and keep the traceability current.

## Architecture and implementation requirements

- Use a clean, scalable module structure with clear separation among workspace composition,
  reusable visual components, view/session state, document/application services, language
  services, execution workers, persistence, and domain models.
- Commands must resolve an exact context containing the active project, document, document kind,
  ownership, revision, capability set, and platform. A stale or incomplete context must fail closed.
- Treat generated decks and comparisons as immutable snapshots. Treat project-owned decks,
  includes, Verilog-A sources, and automation sources as editable, journaled, conflict-aware
  documents. Treat PDK/vendor sources as read-only unless an explicit copy-to-project workflow is
  completed.
- Bind every compile, validation, execution, comparison, diagnostic, result, and artifact to the
  exact source closure and revision that produced it. A stale receipt cannot authorize a run.
- Project/source switching must reset selection, editor models, language services, diagnostics,
  build/runtime state, and result bindings atomically. Never carry state across projects by index
  or display name.
- Use stable document and source identities. Renames, moves, deletes, external changes, copies,
  imports, and dependency updates must preserve or deliberately migrate references.
- Implement complete document lifecycle behavior: new/open/save/save-as/save-all/close/reopen,
  recent files, dirty state, autosave journal, crash recovery, external-change detection, conflict
  resolution, rename/move/delete/duplicate, encoding, BOM, line endings, invalid bytes, IME,
  clipboard, permissions, and platform-specific file access.
- Implement editor commands and shortcuts as real model operations: undo/redo, clipboard, selection,
  multi-cursor, column selection, indentation, comments, formatting, line movement/duplication/
  deletion, folding, navigation, and command enablement.
- Implement workspace and document search/replace with all specified scopes, options, bounded
  result sets, cancellation, stale-result handling, read-only handling, preview, atomic replace,
  undoability, and actionable failure states.
- Implement language services appropriate to each document kind: tokenization, parsing, completion,
  hover, signature help, go-to-definition, references, rename, symbols, quick fixes, formatting,
  and canonical diagnostics. Unsupported semantics must be explicit and fail closed.
- Use one canonical diagnostic model with stable identity, severity, source, code, message, related
  information, exact document/range/revision, and projections into editor markers, Problems,
  navigator badges, status summaries, build output, and automation/debug output.
- Netlist import and normalization must be staged and transactional. Detect dialect/encoding/line
  endings, resolve includes and libraries, expose transformations and unsupported semantics,
  preserve source evidence, preview the result, and commit only after validation succeeds.
- Netlist semantic tooling must operate on the real netlist parser/model and source graph. It must
  handle includes, libraries, parameters, subcircuits, models, control blocks, expressions,
  hierarchy, connectivity, source maps, dialect differences, and error recovery without silently
  changing circuit meaning.
- Verilog-A must support arbitrary multi-file source closures, standard includes, project/vendor
  dependencies, real IEEE 1800.2 parsing and semantic diagnostics, deterministic compilation,
  platform-qualified native and browser backends, cancellation, stale-build rejection, source
  maps, and reproducible build receipts. Native JIT capability must never be implied on browser.
- Automation must support arbitrary project file layouts and helper modules. Entry points, run plans,
  dependency locks, and permission manifests are roles resolved by project configuration and/or
  real runtime semantics, not reserved demo filenames. Validation must use a real Python parser and
  the versioned RSpice automation API; execution and debugging must use a real governed runtime.
- The automation runtime must enforce declared file, network, process, environment, clipboard, and
  project/result capabilities at the host boundary. It must support cancellation, time/resource
  limits, structured output, tracebacks, breakpoints (including conditional/log/hit-count), stack
  frames, locals/globals, watches, exception policy, restart, and recovery after worker failure.
- Never discover or execute Python from the user's system, `PATH`, registry, package manager, or
  development environment. Native distributions must ship signed, content-addressed, app-local
  CPython 3.14 and its standard library in an isolated worker environment. Browser distributions
  must self-host the closed Pyodide 314.0.2 inventory in a dedicated Web Worker. There is no
  optional system-interpreter mode: one product-managed runtime boundary keeps validation,
  debugging, reproducibility, support, and signoff semantics identical for every customer.
- Dependency installation and environment materialization must be deterministic and integrity
  checked. Normal product workflows must not require users to install Python or fetch an interpreter.
  Browser-incompatible native extension dependencies must produce a precise compatibility error.
- Every visible button, menu item, tab, filter, table/tree action, editor control, dialog, shortcut,
  context menu, and navigation target must work as labeled. Disabled actions need a real,
  discoverable reason.
- Never create placeholder panels, fake success, data-only simulations, inert controls, TODOs,
  temporary compatibility layers, or "coming soon" elements in the reachable product UI.
- Domain operations must produce real validated state transitions, persist correctly, participate
  in undo/redo where appropriate, and report actionable errors. Never claim import, validation,
  compilation, execution, save, export, or artifact generation succeeded unless it did.
- Handle malformed files, missing/inaccessible/cyclic includes, duplicate definitions, unsupported
  constructs, incompatible models/packages, read-only sources, conflicting names, partial imports,
  stale revisions, worker crashes, permission denial, quota exhaustion, filesystem failures, and
  browser storage loss without corrupting workspace state.
- Tables, trees, Problems, search results, logs, and editors must be virtualized or otherwise bounded
  according to the accepted scale contract. Filtering, sorting, selection, expansion, keyboard
  navigation, and context actions must remain synchronized.
- Accessibility is a functional requirement: semantic names/roles/states, logical focus order,
  full keyboard operation, visible focus, sufficient contrast, non-color-only diagnostics, zoom,
  reduced motion, screen-reader announcements, and localization-safe layouts.

## Quality process

"Static analysis" includes manual review against the mockup for sizing, spacing, typography,
color, alignment, overflow, interaction states, state correctness, and complete functionality. It
does not mean only compiler or lint checks.

For each implementation slice:

1. Review code, state ownership, command routing, failure modes, and layout against the contracts.
2. Fix obvious behavior, data-integrity, spacing, sizing, state, and accessibility defects.
3. Run formatting, compilation, lint/static checks, focused unit tests, and integration tests.
4. Run the relevant native and browser tests.
5. Inspect the implemented slice visually only after code review and automated checks pass.
6. Exercise realistic interaction sequences, not just initial screenshots.
7. Test common desktop viewports and narrower desktop/tablet-like widths without a phone redesign.
8. Test empty, loading, populated, invalid, stale, conflicted, denied, read-only, offline, cancelled,
   worker-failed, storage-failed, and recovery states as applicable.
9. Fix every discovered defect and add regression coverage before moving to the next slice.

Use focused commands and reasonable timeouts. Do not let a silent build or test run consume many
minutes without investigation.

## Visual inspection coverage

- initial Netlist, Verilog-A, and Automation workspaces
- populated, empty, loading, failure, read-only, and recoverable states
- project/source/document changes and persistence after reopening
- editor tabs, dirty/conflict markers, splitters, minimaps, gutters, folding, diagnostics, and long
  lines/files/paths
- search, replace, filters, result limits, stale results, and navigation
- menus, context menus, shortcuts, toolbars, dialogs, overlays, validation, and disabled reasons
- netlist import/normalization/comparison and source-owner navigation
- multi-file Verilog-A dependency/build/cancellation/stale-receipt flows
- automation file/project management, environment resolution, permissions, validation, run, cancel,
  debugger, worker failure/restart, structured output, and result/artifact navigation
- resizing, scrolling, keyboard focus, zoom, contrast themes, and tablet-like widths

## Testing and evidence

- Add focused unit and integration tests for domain behavior, command wiring, persistence, parser and
  compiler services, runtime protocol, worker isolation, capability enforcement, stale-state
  rejection, and UI state transitions.
- Add regression coverage for every defect discovered.
- Verify native and web targets compile and run their applicable tests.
- Validate mockup/contract synchronization with the repository's existing test suite.
- Perform visual regression and keyboard/accessibility inspection for every reachable surface.
- Qualify scale and recovery against the accepted limits and failure-injection scenarios.
- Do not modify unrelated failing CI unless these changes caused the failure.
- Before committing, run `git diff --check`, inspect the complete scoped diff, and ensure no unrelated
  file is staged.

## Communication

- Give concise progress updates while working and report an honest completion estimate when useful.
- Explain a required design or scope change and obtain approval before implementing it.
- Do not declare completion based on visual similarity or a passing compile. Completion requires
  full mockup parity, generalized data models, complete command/action wiring, real parser/compiler/
  runtime behavior, persistence, validation, accessibility, recovery, scale qualification, tests,
  and successful native/browser verification.

## Completion criteria

The Netlist & Script Editor workspace is complete only when every surface and reachable interaction
in the latest approved mockup and qualified contracts is implemented, visually faithful, stable,
fully wired to real generalized behavior, persisted, accessible, tested, and free of placeholders
or known defects. Provide a final inventory of completed surfaces and workflows, requirement
traceability, tests and qualification evidence, remaining limitations (which must be zero for an
unqualified "complete" claim), commits, and pushed revisions.
