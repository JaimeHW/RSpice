# design/ — design artifacts

Static HTML mockups and brand sources. Nothing here ships; when a design
lands it is ported deliberately (app surfaces into `crates/rspice-ui`,
site pages into `site/`). Mockups are plain HTML — open directly in a
browser, or serve the repo root (`design-repo-root` launch config,
port 8742) and browse to `design/app/…`.

| Folder | Contents |
|---|---|
| `app/` | VOLTA IDE mockups — the design of record for GUI surfaces |
| `website/` | rspice.app marketing-site mockups |
| `internal/` | internal tooling (validation/parity dashboard) |
| `brand/` | Run mark spec + icon source SVGs; rendered set in `brand/export/` |
| `tools/` | `export_icons.py` — rasterizes `brand/` SVGs (expects `brand/` as sibling) |
| `archive/` | superseded explorations kept for their rationale |

Naming: `volta-*` are IDE / design-system artifacts (VOLTA is the app's
design system); `rspice-*` are product-level (website, internal tooling).
Code doc comments in `crates/rspice-ui` cite these files by exact path
(`design/app/volta-….html`, often with a `§` section) — if you move or
rename a file, update the citations.

## app/ — status

| File | Design of record for | Superseded parts |
|---|---|---|
| `volta-dialogs.html` | Modal grammar §01, confirmations §02, simulation options §04, instance properties §05, model browser §06, small forms (New Cell/New View) §07, probe toasts §08, Nyquist/Smith/PZ viewer grammar §09 | §03 add-analysis modal → `volta-simulate-v2.html` palette |
| `volta-dialogs-v2.html` | Calculator §1, PDK/library paths §2, Verilog-A loader §3 | §4 add-analysis modal → `volta-simulate-v2.html` (marked in-file) |
| `volta-simulate-v2.html` | Simulate workspace: analyses card, anchored analysis palette, per-analysis typed forms (all analyses incl. RF/statistical), run-history rail | — |
| `volta-results-workspace.html` | Viewer strip grammar: WAVES, BODE, FFT, EYE, HIST; docbar; shell chrome as static context | Run/overlay architecture → `volta-results-v2.html` |
| `volta-results-v2.html` | Run-keyed results: run shelf, overlay grammar (signal=hue, run=weight), specs matrix, MC dashboard, noise contributors, OP inspector | — |
| `volta-netlist-editor.html` | Netlist editor, parameter tuner, run bar, scope breadcrumb, completion popover | — |
| `volta-schematic-rail.html` | Schematic left rail: navigator, library palette, place strip, nameplate | — |
| `volta-license-dialog.html` | License activation dialog. UX spec: license-key spec §5 (internal doc) | — |
| `volta-symbols.html` | Component symbol set: round-1 judging sheet + shipped symbol gallery | — |
| `volta-touch.html` | Tablet/phone concept, round 2 (landscape thumb-rail; portrait rejected). Not implemented | — |
| `volta-schematic-editor.html` | Desktop schematic editor: document bar (breadcrumb + ERC pill), canvas vocabulary, selection, placement, wiring, checks overlay, hierarchy, context menus (spec), empty state, full keymap | — |
| `volta-library-manager.html` | Library workspace view: three-column browser, docbar, metadata strip, New cell + New view forms (completes volta-dialogs §07), delete confirmations, empty states | — |
| `volta-app-chrome.html` | Shell chrome as first-class surfaces: all nine menus item-for-item, toolbar, workspace tabs (×3 directions), status bar with responsive priorities, console, empty-state grammar | — |
| `volta-app-dialogs.html` | App-level surfaces: command palette (live ranking), Preferences, Keyboard shortcuts (complete deck), About | — |

## website/ · internal/ · brand/ · archive/

- `website/rspice-website.html` — landing page (hero, analyses, platforms,
  pricing, CTA). `website/rspice-website-interior.html` — download, docs
  shell, changelog pages. The deployed site lives in `site/`; these are
  the specs it was ported from.
- `internal/rspice-parity-dashboard.html` — validation-evidence dashboard
  (suite table, oracle overlay drill-down, methodology).
- `brand/run-mark-refined.html` — the Run mark spec of record, including
  the size-cut tiering that `tools/export_icons.py` implements.
  `brand/og-card.html` — social card source for `brand/export/og-card.png`.
- `archive/run-mark-terminals.html` — the terminal-treatment exploration
  behind the 2026-06-11 mark recut; the conclusions live in
  `brand/run-mark-refined.html`, the reasoning only here.

## Coverage

The 2026-06-12 audit found nine GUI surfaces with no design artifact
(preferences, command palette, about, shortcuts help, Library Manager,
desktop schematic editor, console, menu dropdowns, context menus/empty
states) and three specified only as "static context" chrome. All were
closed the same day by the four files added to the table above:
`volta-schematic-editor.html`, `volta-library-manager.html`,
`volta-app-chrome.html`, `volta-app-dialogs.html`.

Inside those files, `ships` tags mark what the code does and `spec`
tags mark proposals. The same-day implementation pass shipped most of
the original backlog: canvas context menus, finding cycling
(F4/Shift+F4), the clickable ERC pill and docbar selection echo,
palette recents/match marks/context verbs, the complete searchable
shortcut deck, About's license line and Copy diagnostics, the Check
menu's View/Clear violations, library read-only marks and named empty
states, and the empty-sheet copy fix. Still open as `spec`: console
source links, schematic autosave, library keyboard navigation and cell
copy/rename, open-any-view routing, the read-only descend banner, and
real Technology/Path metadata.
