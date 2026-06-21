# Schematic Rail And Editor Polish Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Bring the schematic rail, project/library placement area, and center schematic editor into tighter conformance with `design/app/volta-schematic-rail.html` and `design/app/volta-schematic-editor.html`.

**Architecture:** Keep the existing egui shell/view architecture. Add reusable row-highlight support in `TreeRow`, wire token constants in the drawing/palette layer, and focused rail helpers for search, preview actions, net probe dots, and library-cell interface lookup. Resolve shortcut conflicts by preserving canvas `E` properties and `Shift+E` descend behavior from the editor design while improving rail-visible descend affordances.

**Tech Stack:** Rust, egui, existing `rspice-ui` state/shell/view modules, existing cargo test suite.

---

### Task 1: Tree Row Search Highlighting

**Files:**
- Modify: `crates/rspice-ui/src/ui/widgets/tree.rs`

- [x] **Step 1: Write failing tests**

Add tests for a pure helper that returns ASCII case-insensitive match ranges:

```rust
#[test]
fn highlight_ranges_find_case_insensitive_occurrences() {
    assert_eq!(highlight_ranges("Rload rLOAD", "load"), vec![1..5, 7..11]);
}

#[test]
fn highlight_ranges_skip_empty_or_missing_queries() {
    assert!(highlight_ranges("Rload", "").is_empty());
    assert!(highlight_ranges("Rload", "cap").is_empty());
}
```

- [x] **Step 2: Run test to verify it fails**

Run: `cargo test -p rspice-ui --lib ui::widgets::tree::tests`
Expected: FAIL because `highlight_ranges` does not exist.

- [x] **Step 3: Implement widget support**

Add `highlight_query: Option<&'a str>` to `TreeRow`, a builder method, and highlighted layout-job rendering for label and metadata while preserving truncation.

- [x] **Step 4: Run focused tests**

Run: `cargo test -p rspice-ui --lib ui::widgets::tree::tests`
Expected: PASS.

Evidence (2026-06-17): `cargo test -p rspice-ui --lib ui::widgets::tree::tests -- --nocapture` passed 4 tests, including empty query, missing query, ASCII case-insensitive matching, and multiple occurrences.

### Task 2: Wire Token Fidelity

**Files:**
- Modify: `crates/rspice-ui/src/ui/palette.rs`
- Modify: `crates/rspice-ui/src/schematic/view/drawing.rs`

- [x] **Step 1: Write failing tests**

Add tests that assert `INSTRUMENT_DARK.wire == Color32::from_rgb(0x7F, 0xBF, 0x95)` and default wire width equals `1.1`.

- [x] **Step 2: Run test to verify it fails**

Run: `cargo test -p rspice-ui --lib ui::palette schematic::view::drawing`
Expected: FAIL on the dark wire token or width.

- [x] **Step 3: Implement token and width changes**

Set the Instrument dark wire token to `#7FBF95` and use a named default wire width constant of `1.1`.

- [x] **Step 4: Run focused tests**

Run: `cargo test -p rspice-ui --lib ui::palette schematic::view::drawing`
Expected: PASS.

Evidence (2026-06-17): the plan's combined cargo filter syntax is not accepted by cargo, so the filters were run separately. `cargo test -p rspice-ui --lib ui::palette -- --nocapture` passed `instrument_wire_tokens_match_volta_schematic_design_refs`; `cargo test -p rspice-ui --lib schematic::view::drawing -- --nocapture` passed `wire_strokes_use_design_default_width_without_changing_state_widths`.

### Task 3: Rail Integration And Preview Polish

**Files:**
- Modify: `crates/rspice-ui/src/shell/panels/schematic.rs`

- [x] **Step 1: Write failing tests**

Add tests for rail helper behavior:

```rust
#[test]
fn net_probe_dot_tracks_active_highlighted_net() {
    let mut ids = std::collections::HashSet::new();
    ids.insert(42);
    assert!(net_row_is_probed(&[42], &ids, true));
    assert!(!net_row_is_probed(&[42], &ids, false));
    assert!(!net_row_is_probed(&[7], &ids, true));
}

#[test]
fn preview_place_label_matches_design() {
    assert_eq!(preview_place_label(), "Place \u{23ce}");
}
```

- [x] **Step 2: Run test to verify it fails**

Run: `cargo test -p rspice-ui --lib shell::panels::schematic::tests`
Expected: FAIL because helpers do not exist or label is still `Place`.

- [x] **Step 3: Implement rail polish**

Apply row highlighting to instance, net, port, library, and place-typeahead rows. Add active net probe dots. Make preview Place an accent button labelled `Place ⏎`. Change place typeahead Enter handling to work while the input remains focused. Add explicit row-level descend affordance for hierarchical instances without stealing canvas `E`.

- [x] **Step 4: Run focused rail tests**

Run: `cargo test -p rspice-ui --lib shell::panels::schematic::tests`
Expected: PASS.

Evidence (2026-06-17): `cargo test -p rspice-ui --lib shell::panels::schematic::tests -- --nocapture` passed 6 tests, covering preview label, net probe dot behavior, navigator counts, empty editable library visibility, master status, and rail width overflow.

### Task 4: Verification And Review

**Files:**
- No additional files expected.

- [x] **Step 1: Format**

Run: `cargo fmt --check`
Expected: PASS.

- [x] **Step 2: Focused tests**

Run:

```powershell
cargo test -p rspice-ui --lib ui::widgets::tree::tests
cargo test -p rspice-ui --lib shell::panels::schematic::tests
cargo test -p rspice-ui --lib
```

Expected: PASS.

Evidence (2026-06-17): `cargo fmt --all -- --check` passed. `cargo test -p rspice-ui --lib ui::widgets::tree::tests -- --nocapture`, `cargo test -p rspice-ui --lib shell::panels::schematic::tests -- --nocapture`, and `cargo test -p rspice-ui --lib` passed; the full UI library suite reported 266 passed, 0 failed.

- [x] **Step 3: Subagent review**

Dispatch a spec-compliance/code-quality review subagent over the final diff. Fix Critical and Important findings before completion.

Evidence (2026-06-17): subagent `019ed76e-0457-7742-b8d7-3ec0d681bc36` returned `STATUS: PASS` with no spec gaps. It verified tree row highlighting, wire token/default width, rail highlighting, preview `Place \u{23ce}`, net probe dots, focused Enter handling, library-cell lookup, descend affordance, and the plain `E` / `Shift+E` shortcut split.
