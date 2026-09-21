# RSpice drawing-sheet and hardcopy release qualification

## Purpose

This procedure is the release gate for RSpice drawing sheets, Page Setup,
preview, export, and printing. A release is not qualified by a successful
build or by one visual inspection. Every applicable automated and manual gate
below must have retained evidence tied to the exact release commit and binary
digest.

Mobile and tablet qualification is deferred until that product work is
reactivated. Deferral does not waive desktop or browser gates and must not be
reported as mobile support.

## Evidence record

Retain one immutable qualification record containing:

- release version, commit, build provenance, binary/WASM digests, and date;
- operating-system, browser, printer, driver, firmware, paper, and locale
  versions used by every test;
- the exact authored project and signed organization preset package;
- command output, generated artifacts, screenshots, spool/job evidence, and
  measured physical results;
- pass, fail, not-applicable, and blocked dispositions with reviewer identity;
- every deviation, its approved authority, expiry, and corrective-action link;
- independent reviewer approval for physical scale, document control, and
  governed-preset results.

Screenshots alone are insufficient evidence for dimensions, source identity,
artifact bytes, signatures, printer acceptance, or accessibility semantics.

## Automated release gates

Run from a clean release worktree with the supported Rust toolchain. Do not
raise architecture ratchets, skip sheet tests, or reuse an older browser
package to obtain a pass.

First create the focused, checksummed drawing-sheet record on each matching
native host and for the browser target. The runner refuses unknown targets,
cross-host native qualification, an existing evidence path, a dirty release
worktree, source changes during the run, a vacuous test filter, or an
over-budget sheet/hardcopy source unit.

```text
python tools/ci/test_qualify_drawing_sheet.py -v
python tools/ci/qualify_drawing_sheet.py --target <host-target> --out target/drawing-sheet-qualification-<host-target>.json
python tools/ci/qualify_drawing_sheet.py --target wasm32-unknown-unknown --out target/drawing-sheet-qualification-wasm32-unknown-unknown.json
```

The manual `Drawing-sheet qualification` GitHub Actions workflow runs this
contract on x86-64 and Arm64 Windows, Linux, and macOS hosts plus WASM. Retain
each JSON record, its `.sha256` file, its log directory, and the workflow run.
An `--allow-dirty` result is labeled `development-pass` and is never eligible
as release evidence. The focused record is necessary but does not replace the
complete clean-commit product CI gates below.

```text
cargo test --locked -p rspice-core --test save_directives
cargo test --locked -p rspice-ui --lib
cargo test --locked -p rspice-sheet-publisher
cargo test --locked -p rspice-ui --test module_layering
cargo check --locked -p rspice-ui --target wasm32-unknown-unknown
cargo check --locked -p rspice-ui --target x86_64-pc-windows-msvc
cargo check --locked -p rspice-ui --target x86_64-unknown-linux-gnu
```

The macOS build and tests must run on the supported Apple build host; a Rust
target installed on Windows is not macOS qualification. Retain the complete
CI result for each supported architecture.

The automated evidence must cover at least:

- exact micrometre geometry, standard/custom paper limits, margins, bleed,
  printable area, orientation, rotation, zones, and title-block placement;
- project-default inheritance, per-sheet overrides, transaction rollback,
  undo/redo, stable sheet identity, sheet ordering, and mixed sheet sets;
- title resolution, revision/document-control fields, Unicode, long values,
  managed logos, signed presets, trust roots, revocation, and tamper rejection;
- canvas, preview, PDF/PDF-A, SVG, raster, browser-worker, and native-printer
  parity from the same sealed source and plan;
- clipping versus extent expansion, crop/registration marks, borders, zones,
  grids, legends, headers, watermarks, and searchable embedded text;
- deterministic digests, bounded worker transfers, cancellation, stale-result
  rejection, truthful receipts, and fail-closed publication errors;
- every supported standard size, orientation, and title contract in the
  drawing-sheet publication matrix.

Any flaky, timed-out, ignored, filtered, or environment-denied in-scope test is
not a pass. Investigate it or record the release as blocked.

## Golden qualification project

Use a retained project whose source digest is recorded before testing. It must
contain:

- at least four sheets with stable non-label identities and deliberate
  reordering;
- ISO A4 and ANSI Letter sheets, one large-format sheet, portrait and
  landscape orientations, and an exact custom size;
- project-default, inherited, overridden, and signed organization formats;
- clockwise and counter-clockwise title-block rotation where supported;
- every document-control field, long Unicode text, and a managed vector logo;
- content inside the sheet, exactly on the boundary, and outside every edge;
- dense wires, fine strokes, color traces, dash redundancy, junctions, text,
  and symbols at the supported glyph boundary;
- a mixed-size sheet set and enough pages to verify numbering and collation.

Keep the project read-only during artifact comparison. Any authored mutation
requires a new source digest and invalidates earlier evidence.

## Desktop Page Setup qualification

On Windows, macOS, and Linux:

1. Open Page Setup from the exact active sheet and record the displayed
   library/cell/view, stable sheet identity, ordinal, inherited source, and
   working revision.
2. Exercise every page, keyboard path, invalid value, unit conversion, and
   scope choice. Verify focus, error placement, disabled reasons, and no
   clipped controls at the supported desktop sizes and scale factors.
3. Change size, orientation, margins, bleed, frame/zones, title block, and
   inheritance. Verify the live preview and change summary update together.
4. Cancel and prove the authoritative project bytes and undo history did not
   change. Apply once and prove one transaction, one revision, and one undo
   entry were created. Undo and redo must restore exact bytes.
5. Reorder and rename sheets, close/reopen the project, and prove setup stays
   bound to stable identity rather than label or ordinal.
6. Import valid, unknown-key, revoked-key, tampered, stale-schema, duplicate,
   and oversized preset packages. Only the valid governed package may commit.

## Browser qualification

Build a fresh optimized WASM package and serve it over the supported
deployment headers. Test every supported browser/OS pair with hardware WebGPU.

- Repeat Page Setup apply/cancel, inherited/override, preview, current-sheet,
  and mixed-sheet-set scenarios.
- Verify worker startup, cancellation, source/plan generation guards, bounded
  transfers, and zero console errors or unhandled rejections.
- Verify Print resolves the exact governed active sheet immediately before
  invoking the browser dialog. Inspect the browser print preview manually;
  browser automation must not be used as proof that the OS dialog printed.
- Verify PDF/SVG/raster downloads, filenames, media types, byte digests, page
  counts, text search, and re-opened artifact geometry.
- Repeat at 100%, 125%, 150%, and 200% browser zoom. Canvas zoom must not alter
  authored or exported physical dimensions.

## Physical printer matrix

At minimum qualify one current device/driver in each shipping path:

- Windows vendor PCL6 or PostScript driver;
- macOS vendor or AirPrint driver;
- Linux CUPS/IPP driver;
- monochrome and color output;
- Letter and A4; large format and custom media when advertised as supported;
- simplex/duplex, collated copies, named tray, manual feed, and unsupported
  option rejection.

For every row:

1. Capture printer identity, capability digest, driver, firmware, media,
   resolution, duplex, copies, collation, and source tray before submission.
2. Print a 1:1 calibration page with orthogonal 100 mm rulers and registration
   marks. Measure both axes with calibrated equipment. The result must meet the
   product scale requirement and the printer vendor specification; automatic
   fit/shrink is a failure unless explicitly selected by the user.
3. Verify margins, bleed/clipping, crop marks, border, zones, title block,
   logo, fine strokes, grayscale/dash redundancy, and readable small text.
4. Verify page order, sheet/page labels, mixed orientation, duplex edge,
   collation, copies, and tray selection against the sealed plan.
5. Exercise offline printer, paper mismatch, out-of-paper, cancellation before
   acceptance, cancellation after partial completion, driver rejection, and
   spooler restart. UI state and receipt must never claim pages not accepted.
6. Compare a retained printer raster or spool artifact with the preview at the
   semantic level. Device halftoning differences are allowed; missing,
   substituted, clipped, or reordered content is not.

Virtual PDF/XPS printers do not satisfy the physical-printer gate.

## Artifact inspection

Independently inspect every shipped output format:

- page boxes and physical dimensions are exact for each sheet;
- PDF/A conformance passes the approved external validator;
- fonts are embedded or outlined according to the selected policy;
- searchable text, Unicode, links, metadata, and document-control values are
  correct and no private runtime data is present;
- vector output stays vector-safe, raster resolution and alpha are correct,
  and multi-part packages contain only the authenticated manifest and parts;
- two publications from identical source, plan, build, and timestamp contract
  are byte-identical where determinism is promised;
- corrupted, truncated, reordered, substituted, or oversized worker/artifact
  payloads are rejected without partial publication.

## Accessibility and interaction qualification

On every desktop OS, complete the workflow using keyboard only and the
supported screen reader (NVDA/JAWS, VoiceOver, and Orca as applicable).

- Dialog title, purpose, active sheet, steps, fields, errors, preview status,
  busy state, progress, cancellation, and final outcome must be announced.
- Focus must enter predictably, remain trapped only while modal, return to the
  invoking command, and never disappear after validation or worker completion.
- High contrast, 200% UI scale, reduced motion, light/dark themes, and long
  localized labels must retain visible focus and unclipped critical controls.
- Canvas-only browser feedback must be tested according to the documented
  browser accessibility limitation and must not be represented as a full
  accessibility-tree substitute.

Automated accessibility metadata tests are necessary but do not replace this
assistive-technology gate.

## Release decision

Release approval requires:

- all applicable automated commands green on clean supported hosts;
- no unresolved severity-1 or severity-2 sheet/hardcopy defect;
- signed independent review of the golden project, physical scale, artifact
  inspection, governed-preset ceremony, and accessibility results;
- every blocked or not-applicable row explicitly accepted by release authority;
- no claim for a deferred platform or unqualified printer/browser path.

If any required evidence is missing or indirect, the sheet system is not yet
qualified for production release, even when the implementation appears
complete.
