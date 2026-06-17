# Symbol Viewer Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the designed symbol view surface inside the existing Schematic-family workspace.

**Architecture:** Symbol views remain `ViewType::Symbol` documents opened from the Library into the existing Schematic workspace tab. Persistent symbol geometry lives in the symbol view metadata, while the paired schematic view supplies the port contract used by the pin rail, pill states, and generated defaults.

**Tech Stack:** Rust, egui/eframe, serde metadata stored on `crate::state::View`, existing `SchematicState::interface_ports()` and `generate_symbol()` geometry.

---

### Task 1: Routing And Workspace Safety

**Files:**
- Modify: `crates/rspice-ui/src/state/workspace.rs`
- Modify: `crates/rspice-ui/src/common/app/app_workspace_actions.rs`
- Modify: `crates/rspice-ui/src/shell/views/library.rs`

- [x] Write tests proving `ViewType::Symbol` opens as an active workspace document without creating or overwriting a `schematic_buffers["lib/cell/symbol"]` entry.
- [x] Update workspace buffer creation so only schematic-like views allocate schematic buffers.
- [x] Update app workspace sync/open logic so switching through a symbol view preserves the last real schematic buffer and derives read-only state from the active library.
- [x] Update Library openability rules so symbol rows advertise and perform `double-click to open`.

### Task 2: Symbol Document Model

**Files:**
- Create: `crates/rspice-ui/src/state/symbol.rs`
- Modify: `crates/rspice-ui/src/state/mod.rs`

- [x] Add tests for metadata round-trip, generated-from-schematic construction, pin pill summaries, off-grid imported pin detection, and additive port updates.
- [x] Add `SymbolDocument`, `SymbolPin`, `SymbolShape`, `SymbolLabelAnchors`, and `PinSummary`.
- [x] Serialize the document as JSON in view metadata under a versioned key.
- [x] Generate a default rectangular symbol from schematic ports using direction-aware placement and 40-unit terminal lattice coordinates.

### Task 3: Symbol Surface Rendering

**Files:**
- Create: `crates/rspice-ui/src/shell/views/symbol.rs`
- Modify: `crates/rspice-ui/src/shell/views/mod.rs`
- Modify: `crates/rspice-ui/src/shell/views/schematic.rs`

- [x] Branch the Schematic-family center view on `workspace.active_view_type()`.
- [x] Render `lib · CELL · symbol` docbar with the pin pill.
- [x] Reuse the schematic read-only banner grammar and copy-cell action for read-only libraries.
- [x] Render symbol canvas, terminal grid, bbox, origin, labels, placed/unplaced/orphan pins, as-placed preview tile, and pins rail.

### Task 4: Symbol Interactions

**Files:**
- Modify: `crates/rspice-ui/src/shell/views/symbol.rs`
- Modify: `crates/rspice-ui/src/common/app/app_actions.rs`

- [x] Implement selection, `P` place-next-pin, click-to-select pin, drag-to-move pin, `Esc` cancel, `F` fit, zoom, and pan.
- [x] Gate mutating symbol interactions behind the same read-only refusal console line.
- [x] Keep schematic-only commands from mutating stale schematic state while a symbol view is active.
- [x] Persist every symbol edit to metadata and mark the active open view dirty.

### Task 5: Verification And Review

**Files:**
- Validate the full Rust UI crate and a live GUI render.

- [x] Run the targeted symbol/workspace tests after each red/green slice.
- [x] Run `cargo test -p rspice-ui`.
- [x] Run `cargo build -p rspice-ui`.
- [x] Launch `target/debug/rspice-ui.exe`, open a symbol view, and capture a fresh screenshot.
- [x] Review the diff against `design/app/volta-symbol-editor.html` and record final verification evidence.

Final verification:
- `cargo check -p rspice-ui`
- `cargo test -p rspice-ui` (154 passed)
- `cargo build -p rspice-ui`
- Visual QA screenshot: `diagnostics/symbol-viewer-surface-reviewed.png`
