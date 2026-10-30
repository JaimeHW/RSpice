#!/usr/bin/env python3
"""Ratchet the coupling between the slices rspice-app is being split into.

Every `.rs` file under crates/rspice-app/src belongs to the slice of the first
SLICE_RULES pattern its path matches. Three groups are measured: references
between two feature slices; references from a lower slice to a feature or the
shell, and from model to any other slice; and whole-app counts (inherent
`impl RSpiceApp` methods outside the shell, inherent `impl AppState` methods
outside the session aggregate, and functions outside the shell whose signature
takes `&RSpiceApp` or `&mut RSpiceApp`, one of each per function).

Method: a small Rust lexer tracks items, inline modules and `impl` blocks.
Production code only: a file reached solely through `#[cfg(test)] mod x;` (or
reached by no module and named like a test) and every `#[cfg(test)]`/`#[test]`
item are skipped. A reference is a `crate::`/`super::`/`self::` path, use-tree
leaves included, resolved through module files and `use` re-exports to its
defining file; or a use of a public free item name of 4+ characters that one
slice alone defines, where the file imports it, globs it in, or qualifies it
by a crate path. A token found both ways counts once. Macro bodies and trait
calls are opaque, so a count is a floor that moves only with the source. A
slice already moved into a crate of its own (LOWER_CRATES) is no longer measured,
but its definitions and module names still count as that slice's.

app_slice_ceilings.json holds every non-zero measurement, and each must equal
its ceiling: above it, or an edge with no entry, is new coupling; below it, the
ceiling is lowered in the same change; at zero, the entry is deleted.
`--write-ceilings` rewrites the file from the measurement.
"""

from __future__ import annotations

import argparse
import collections
import json
import posixpath
import re
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "crates" / "rspice-app" / "src"
CEILINGS = Path(__file__).resolve().with_name("app_slice_ceilings.json")
# Slice -> the src directory of the crate its files were moved into: a lower
# crate, or a feature's own.
LOWER_CRATES = {
    "model": ROOT / "crates" / "rspice-app-state" / "src",
    "services": ROOT / "crates" / "rspice-app-services" / "src",
    "run": ROOT / "crates" / "rspice-app-runs" / "src",
    "framework": ROOT / "crates" / "rspice-app-workbench" / "src",
    "sheets": ROOT / "crates" / "rspice-app-sheets" / "src",
    "settings": ROOT / "crates" / "rspice-app-settings" / "src",
}

LOWER = ("model", "services", "run", "framework")
FEATURES = (
    "project", "design", "simulate", "results", "models",
    "code", "sheets", "hardcopy", "cloud", "settings",
)
SESSION = "session"  # the AppState/WorkbenchSession aggregate
SHELL = "shell"
SLICES = (*LOWER, SESSION, *FEATURES, SHELL)
GROUPS = ("feature->feature", "lower->upper", "whole-app")

W = "^workbench/"
D = "^workbench/app/dialogs/"
# Source path (relative to src/) -> slice. First match wins.
SLICE_RULES = [
    (r"^(lib|main|workbench)\.rs$", "shell"),
    (r"^feature_tests(\.rs$|/)", "shell"),
    (W + r"app\.rs$", "shell"),
    (W + r"app/browser_history\.rs$", "shell"),
    (W + r"frame\.rs$", "shell"),
    (W + r"frame/dialog_order\.rs$", "shell"),
    # The four session aggregates and their init/serialization.
    (W + r"app_state\.rs$", "session"),
    (W + r"app_state/session\.rs$", "session"),
    (W + r"app_state/session/(state_init|serialization)\.rs$", "session"),
    (W + r"app_state/stack_budget\.rs$", "session"),
    (W + r"state\.rs$", "session"),
    (W + r"state/tests", "session"),
    (W + r"lifecycle/session\.rs$", "session"),
    (W + r"lifecycle/session/", "session"),
    (D + r"state(\.rs$|/)", "session"),
    # AppState sub-owners.
    (W + r"app_state/model(\.rs$|/)", "model"),
    (W + r"app_state/(sim_setup|design_history|cell_usage|run_identity|symbol_contract)", "model"),
    (W + r"app_state/(design_checks|diagnostic_store|waivers|rule_deck|schematic_preferences)", "model"),
    (W + r"app_state/(problems_view|active_viewer)", "framework"),
    # The canvas's drag, hover and focus are design's; the design slot holds them.
    (W + r"app_state/interaction_state", "design"),
    (W + r"app_state/viewer_capabilities", "results"),
    (W + r"app_state/run_preflight", "run"),
    (W + r"app_state/session/autosave", "services"),
    (W + r"app_state/session/calculator", "results"),
    (W + r"app_state/session/pdk_settings", "models"),
    (W + r"app_state/session/preference_runtime", "shell"),
    (W + r"app_state/session/(preferences_file|shortcut_library)", "settings"),
    (W + r"app_state/session/shortcuts", "framework"),
    (W + r"app_state/session/veriloga_library", "code"),
    (W + r"app_state/", "session"),
    # The slots AppState holds, each by the feature that owns it.
    (W + r"slots(\.rs$|/(defaults_)?tests\.rs$)", "session"),
    (W + r"slots/cloud\.rs$", "cloud"),
    (W + r"slots/code\.rs$", "code"),
    (W + r"slots/design\.rs$", "design"),
    (W + r"slots/hardcopy\.rs$", "hardcopy"),
    (W + r"slots/models\.rs$", "models"),
    (W + r"slots/platform\.rs$", "services"),
    (W + r"slots/project\.rs$", "project"),
    (W + r"slots/results\.rs$", "results"),
    (W + r"slots/runs\.rs$", "run"),
    (W + r"slots/settings\.rs$", "settings"),
    (W + r"slots/sheets\.rs$", "sheets"),
    (W + r"slots/shell\.rs$", "shell"),
    (W + r"slots/simulate\.rs$", "simulate"),
    # Dialogs, by the feature they serve.
    (D + r"(check_and_save|check_and_save_validation|design_management|project_revision_history)(\.rs$|/)", "project"),
    (D + r"project(\.rs$|/)", "project"),
    (D + r"library/(new_veriloga_device|veriloga_import)", "code"),
    (D + r"library(\.rs$|/)", "project"),
    (D + r"(configuration_sets|plan_removal)(\.rs$|/)", "simulate"),
    (D + r"(confirmation|confirmation_state|operation_primitives|review_primitives|window_session)(\.rs$|/)", "framework"),
    (D + r"(connectivity_manager|design_review_comments|engineering_table|find_replace|grid_snap_routing|hierarchy"
         r"|object_properties|placement|property_dialog|replace_instance|rule_setup|selection"
         r"|visibility_options|create_model_bound_symbol|symbol_definition|stimulus_link)(\.rs$|/)", "design"),
    (D + r"(drawing_sheet_\w+|delete_sheet)(\.rs$|/)", "sheets"),
    (D + r"hardcopy(\.rs$|/)", "hardcopy"),
    (D + r"help(\.rs$|/)", "shell"),
    (D + r"(license|live_session|publish_web|unpublish_web)(\.rs$|/)", "cloud"),
    (D + r"(pdk_settings|pdk_workflow)(\.rs$|/)", "models"),
    (D + r"preferences(\.rs$|/)", "settings"),
    (D + r"(source_document|source_find|source_language_tools|veriloga_build_settings)(\.rs$|/)", "code"),
    (W + r"app/dialogs\.rs$", "shell"),
    # Actions and the rest of app/.
    (W + r"app/actions\.rs$", "shell"),
    (W + r"app/actions/export_image", "hardcopy"),
    (W + r"app/actions/file", "project"),
    (W + r"app/actions/(hierarchy_net_highlight|property_edit|stimulus|symbol|workspace)", "design"),
    (W + r"app/actions/reveal", "shell"),
    (W + r"app/actions/sheets", "sheets"),
    (W + r"app/actions/veriloga", "code"),
    (W + r"app/command_palette", "framework"),
    (W + r"app/drawing_sheet_edit", "sheets"),
    (W + r"app/(pack_persistence|pdk_persistence)", "models"),
    (W + r"app/schematic/edit_authority\.rs$", "model"),
    (W + r"app/schematic", "design"),
    (W + r"app/(wiring_acceptance|gesture_tests|touch_flow_tests)", "design"),
    (W + r"app/(source_editor_command_tests|veriloga_\w+_tests)", "code"),
    (W + r"app/accesskit_dump_tests\.rs$", "shell"),
    # Chrome, docks, commands and menus.
    (W + r"chrome/presence", "cloud"),
    (W + r"chrome/(sheet_strip|status_bar/sheets)", "sheets"),
    (W + r"chrome/(document_bar|title_bar|toolbar)/code", "code"),
    (W + r"chrome/(document_bar|status_bar|title_bar|toolbar)/design", "design"),
    (W + r"chrome/(document_bar|toolbar)/models", "models"),
    (W + r"chrome/(document_bar|toolbar)/project", "project"),
    (W + r"chrome/(activity_rail|document_bar|status_bar|toolbar)/results", "results"),
    (W + r"chrome/status_bar/shell", "shell"),
    (W + r"chrome/(document_bar/simulate|toolbar/(run_config_chip|simulate))", "simulate"),
    (W + r"chrome", "framework"),
    (W + r"docks/inspector/design/sheet", "sheets"),
    (W + r"docks/inspector/(design|stimulus|symbol)", "design"),
    (W + r"docks/inspector/(code_workspace|veriloga)", "code"),
    (W + r"docks/inspector/models", "models"),
    (W + r"docks/inspector/project", "project"),
    (W + r"docks/inspector/(result_authority|results|run_details)", "results"),
    (W + r"docks/inspector/(simulation|verify)", "simulate"),
    (W + r"docks/navigator/(design|stimulus|symbol)", "design"),
    (W + r"docks/navigator/models", "models"),
    (W + r"docks/navigator/(netlist|source_bundle|veriloga)", "code"),
    (W + r"docks/navigator/project", "project"),
    (W + r"docks/navigator/results", "results"),
    (W + r"docks/navigator/(simulate|verify)", "simulate"),
    (W + r"docks/navigator/verification_footer", "simulate"),
    (W + r"docks", "framework"),
    (W + r"commands/code_context", "code"),
    (W + r"commands/result_navigation", "results"),
    # The legacy command enum, its tables and its dispatcher: the shell's
    # fallback for every key no registered feature owns.
    (W + r"commands(\.rs$|/)", "shell"),
    (W + r"dispatch\.rs$", "shell"),
    # The shell's list of registered features, and each feature's own home.
    (W + r"features\.rs$", "shell"),
    (W + r"sheets(\.rs$|/)", "sheets"),
    (W + r"settings(\.rs$|/)", "settings"),
    (W + r"hardcopy(\.rs$|/)", "hardcopy"),
    (W + r"results(\.rs$|/)", "results"),
    (W + r"cloud(\.rs$|/)", "cloud"),
    (W + r"models(\.rs$|/)", "models"),
    (W + r"project(\.rs$|/)", "project"),
    (W + r"shell_host(\.rs$|/)", "shell"),
    (W + r"menu_bar/waveform_export", "results"),
    (W + r"menu_bar/netlist_compat", "code"),
    (W + r"menu_bar/netlist_export", "code"),
    (W + r"menu_bar", "shell"),
    (W + r"diagnostic_routes", "shell"),
    # Documents and surfaces.
    (W + r"documents\.rs$", "framework"),
    (W + r"documents/(code_workspace|text_document_model|text_editor_commands|virtual_text_editor|netlist_document)", "code"),
    (W + r"documents/(model_editor|model_correlation)", "models"),
    (W + r"documents/result_document", "results"),
    (W + r"surfaces\.rs$", "framework"),
    (W + r"surfaces/(automation|netlist|veriloga)", "code"),
    (W + r"surfaces/(design|stimulus)", "design"),
    (W + r"surfaces/(library_cellview_manager|project)", "project"),
    (W + r"surfaces/(models|model_editor|model_correlation|model_correlation_controller|pdk_technology_admin)", "models"),
    (W + r"surfaces/results", "results"),
    (W + r"surfaces/(simulate|verify)", "simulate"),
    # Tools, workflows, lifecycle and the rest of workbench/.
    (W + r"tools/(examples_browser|project_launcher)", "project"),
    (W + r"tools/jobs_manager", "simulate"),
    (W + r"tools/(notification_center|specialist_tool_browser)", "framework"),
    (W + r"tools\.rs$", "shell"),
    (W + r"workflows/(file_workflow|project_workflow)", "project"),
    (W + r"workflows/export_workflow", "services"),
    (W + r"workflows/netlist_workflow", "code"),
    (W + r"workflows/(result_import_workflow|result_import_adapters)", "results"),
    (W + r"workflows", "shell"),
    # The shell's transactions over the whole project, run where a feature's
    # work asks for one.
    (W + r"session/", "shell"),
    (W + r"lifecycle/window_session", "framework"),
    (W + r"lifecycle", "services"),
    (W + r"hardcopy_adapters", "hardcopy"),
    (W + r"browser/(file_import|download|qualification|background_alerts|accessibility)", "services"),
    (W + r"browser/navigation", "framework"),
    (W + r"(browser|platform)\.rs$", "services"),
    (W + r"framework(\.rs$|/)", "framework"),
    (W + r"routing", "framework"),
    # The process logger is main's: env_logger and its native init.
    (W + r"logging\.rs$", "shell"),
    (W + r"(layout|localization)\.rs$", "framework"),
    # The workbench's own session, the views over its records, and how a
    # retired document identity reads back.
    (W + r"state/(navigator_tree|workbench_session|console_views|retired_documents)", "framework"),
    (W + r"state/plan_provenance", "results"),
    (W + r"state/session_views", "session"),
    (W + r"state/stimulus_", "design"),
    (W + r"state/workflow_drafts", "simulate"),
    (W + r"shortcuts", "settings"),
    (W + r"preferences\.rs$", "model"),
    (W + r"preferences/drawing_sheet", "model"),
    (W + r"preferences", "settings"),
    (W + r"(account_organization|live_session|user_identity)\.rs$", "cloud"),
    (W + r"cross_probe\.rs$", "design"),
    (W + r"examples/", "shell"),
    (W + r"preflight", "run"),
    (W + r"simulation_analysis_tabs", "simulate"),
    # Top-level modules.
    (r"^state(\.rs$|/)", "model"),
    (r"^source_guard\.rs$", "shell"),
    (r"^(product|quantity|time_compat|source_revision|results|diagnostics|fixture_root)(\.rs$|/)", "model"),
    (r"^simulation/dialog", "simulate"),
    (r"^simulation/(plan|placed_sources|stimulus_realize|table_route)", "model"),
    (r"^simulation(\.rs$|/)", "run"),
    (r"^properties/model_browser", "models"),
    (r"^properties(\.rs$|/)", "design"),
    (r"^schematic/view/sheet_visibility\.rs$", "design"),
    # The sheet frame on the schematic canvas is design's painting: the
    # schematic view paints the sheet every frame.
    (r"^schematic/view/drawing_sheet", "design"),
    (r"^schematic(\.rs$|/)", "design"),
    (r"^services/(cloud_account|license|dpapi)", "cloud"),
    (r"^services/model_qualification", "models"),
    (r"^services/drc_tests", "design"),
    (r"^(services|io)(\.rs$|/)", "services"),
    (r"^(automation_runtime|build_identity)", "services"),
    (r"^automation_workflow", "code"),
    (r"^analysis(\.rs$|/)", "results"),
    (r"^ui\.rs$", "framework"),
]
COMPILED_RULES = [(re.compile(pattern), slice_name) for pattern, slice_name in SLICE_RULES]


def slice_of(path: str) -> str | None:
    """The slice owning a src-relative path, or None when no rule matches."""
    for pattern, slice_name in COMPILED_RULES:
        if pattern.search(path):
            return slice_name
    return None


def edge_group(source: str, target: str) -> str | None:
    """The ratcheted group of a reference between two different slices."""
    if source in FEATURES and target in FEATURES:
        return "feature->feature"
    if source in LOWER and (
        target in FEATURES
        or target in (SESSION, SHELL)
        or (target in LOWER and LOWER.index(target) > LOWER.index(source))
    ):
        return "lower->upper"
    return None


# ---------------------------------------------------------------- test files

MOD_DECL = re.compile(
    r"^(?P<attrs>(?:[ \t]*#\[[^\n]*\][ \t]*\n)*)[ \t]*(?:pub(?:\([^)]*\))?\s+)?mod\s+(?P<name>r#\w+|\w+)\s*;",
    re.M,
)
PATH_ATTR = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
CFG_TEST_ATTR = re.compile(r"#\[\s*cfg\s*\(\s*(?:test\b|all\s*\([^]]*\btest\b)")
INLINE_MOD = re.compile(
    r"^(?P<attrs>(?:[ \t]*#\[[^\n]*\][ \t]*\n)*)(?P<indent>[ \t]*)(?:pub(?:\([^)]*\))?\s+)?"
    r"mod\s+(?P<name>r#\w+|\w+)\s*\{[ \t]*$",
    re.M,
)


def inline_modules(text: str) -> list[tuple[int, int, str, bool]]:
    """(start, end, name, test-only) of each inline `mod name {` block, which closes at the first
    `}` line at its own indent (rustfmt's layout)."""
    blocks = []
    for match in INLINE_MOD.finditer(text):
        close = re.compile(rf"^{re.escape(match.group('indent'))}\}}[ \t]*$", re.M).search(text, match.end())
        if close:
            blocks.append((match.end(), close.start(), match.group("name").replace("r#", ""),
                           bool(CFG_TEST_ATTR.search(match.group("attrs")))))
    return blocks


def test_files(sources: dict[str, str]) -> dict[str, bool]:
    """Whether each file is test-only: reached only through `#[cfg(test)] mod`."""
    children = collections.defaultdict(list)
    for path, text in sources.items():
        directory, _, base = path.rpartition("/")
        child_dir = directory if base in ("lib.rs", "main.rs", "mod.rs") else path[:-3]
        prefix = f"{child_dir}/" if child_dir else ""
        blocks = inline_modules(text)
        for match in MOD_DECL.finditer(text):
            attrs = match.group("attrs")
            name = match.group("name").replace("r#", "")
            # A declaration inside inline modules names a file beneath them.
            enclosing = [block for block in blocks if block[0] <= match.start() < block[1]]
            nested = "".join(f"{block[2]}/" for block in enclosing)
            explicit = PATH_ATTR.search(attrs)
            if explicit:
                candidates = [posixpath.normpath(posixpath.join(directory, explicit.group(1)))]
            else:
                candidates = [f"{prefix}{nested}{name}.rs", f"{prefix}{nested}{name}/mod.rs"]
            found = next((c for c in candidates if c in sources), None)
            if found:
                test = bool(CFG_TEST_ATTR.search(attrs)) or any(block[3] for block in enclosing)
                children[path].append((found, test))

    def reachable(through_tests: bool) -> set[str]:
        found, stack = set(), [root for root in ("lib.rs", "main.rs") if root in sources]
        while stack:
            path = stack.pop()
            if path not in found:
                found.add(path)
                stack.extend(child for child, test in children[path] if through_tests or not test)
        return found

    def named_like_a_test(path: str) -> bool:
        base = path.rpartition("/")[2]
        return base == "tests.rs" or base.endswith("_tests.rs") or base.startswith("test_") or "/tests/" in f"/{path}"

    production, reached = reachable(False), reachable(True)
    return {path: path not in production if path in reached else named_like_a_test(path) for path in sources}


# ---------------------------------------------------------------- lexer

IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
NUMBER = re.compile(r"[0-9][0-9A-Za-z_]*(\.[0-9][0-9A-Za-z_]*)?([eE][+-]?[0-9_]+)?")
STRING_START = re.compile(r'(br|cr|r|b|c)?(#*)"')
PUNCT2 = ("::", "->", "=>", "==", "!=", "<=", ">=", "&&", "||", "+=", "-=", "*=", "/=", "..")
KEYWORDS = frozenset(
    "as async await break const continue crate dyn else enum extern false fn for if impl in let loop match mod "
    "move mut pub ref return self Self static struct super trait true type union unsafe use where while yield "
    "macro_rules default".split()
)


def lex(src: str) -> list[tuple[str, str]]:
    """Tokens as (kind, text), kind in id/p/lit; comments dropped, literals opaque."""
    tokens = []
    i, n = 0, len(src)
    while i < n:
        c = src[i]
        if c in " \t\r\n\f\v﻿":
            i += 1
            continue
        if c == "/" and src.startswith("//", i):
            end = src.find("\n", i)
            i = n if end < 0 else end
            continue
        if c == "/" and src.startswith("/*", i):
            depth, i = 1, i + 2
            while i < n and depth:
                if src.startswith("/*", i):
                    depth, i = depth + 1, i + 2
                elif src.startswith("*/", i):
                    depth, i = depth - 1, i + 2
                else:
                    i += 1
            continue
        start = STRING_START.match(src, i) if c in 'brc"' else None
        if start and (start.group(1) or c == '"') and (start.group(1) or not start.group(2)):
            j = start.end()
            if "r" in (start.group(1) or ""):
                close = '"' + start.group(2)
                end = src.find(close, j)
                i = (n if end < 0 else end) + len(close)
            else:
                while j < n and src[j] != '"':
                    j += 2 if src[j] == "\\" else 1
                i = j + 1
            tokens.append(("lit", "str"))
            continue
        if c == "'" or (c == "b" and src.startswith("'", i + 1)):
            j = i + (2 if c == "b" else 1)
            if j < n and src[j] == "\\":
                end = src.find("'", j + 2)
                i = (n if end < 0 else end) + 1
                tokens.append(("lit", "char"))
                continue
            if j + 1 < n and src[j + 1] == "'" and src[j] != "'":
                i = j + 2
                tokens.append(("lit", "char"))
                continue
            lifetime = IDENT.match(src, j)
            if lifetime:
                i = lifetime.end()
                tokens.append(("p", "'lt"))
                continue
            i += 1
            continue
        if c == "_" or c.isascii() and c.isalpha():
            if src.startswith("r#", i) and i + 2 < n and IDENT.match(src, i + 2):
                i += 2
            word = IDENT.match(src, i)
            tokens.append(("id", word.group(0)))
            i = word.end()
            continue
        if c.isdigit():
            tokens.append(("lit", "num"))
            i = NUMBER.match(src, i).end()
            continue
        if src[i:i + 2] in PUNCT2:
            tokens.append(("p", src[i:i + 2]))
            i += 2
            continue
        tokens.append(("p", c))
        i += 1
    return tokens


# ---------------------------------------------------------------- item scopes

ITEM_SCOPES = ("file", "mod", "impl", "trait")
DEFINING = ("fn", "struct", "enum", "union", "trait", "type", "const", "static", "mod", "macro_rules")
OPENING = ("mod", "impl", "trait", "fn", "struct", "enum", "union", "macro_rules", "extern")


def is_test_attribute(words: list[str]) -> bool:
    text = " ".join(words)
    return (
        words == ["test"]
        or text.startswith("cfg ( test )")
        or text.startswith("cfg ( all ( test")
        or (len(words) >= 3 and words[-1] == "test" and words[-2] == "::")
    )


def impl_self_type(header: list[tuple[str, str]]) -> tuple[str | None, str | None]:
    """(self type, trait) of an `impl` header: the last top-level identifiers."""
    i = 0
    if header[:1] and header[0][1] == "<":
        depth = 0
        while i < len(header):
            depth += {"<": 1, ">": -1}.get(header[i][1], 0)
            i += 1
            if depth == 0:
                break
    body = header[i:]
    depth = 0
    for k, (_, word) in enumerate(body):
        depth += {"<": 1, ">": -1}.get(word, 0)
        if word == "where" and depth <= 0:
            body = body[:k]
            break
    depth, split = 0, None
    for k, (_, word) in enumerate(body):
        depth += {"<": 1, ">": -1}.get(word, 0)
        if word == "for" and depth == 0 and k > 0:
            split = k
            break

    def last_ident(segment: list[tuple[str, str]]) -> str | None:
        depth, last = 0, None
        for kind, word in segment:
            depth += {"<": 1, ">": -1}.get(word, 0)
            if depth == 0 and kind == "id" and word not in (
                "dyn", "mut", "unsafe", "const", "for", "impl", "crate", "super", "self",
            ):
                last = word
        return last

    if split is not None:
        return last_ident(body[split + 1:]), last_ident(body[:split])
    return last_ident(body), None


def scan(tokens: list[tuple[str, str]], file_is_test: bool) -> dict:
    """Walk the item tree.

    Returns per-token context (test flag, inline module path), the item
    definitions, and every `fn` header with the `impl` it sits in.
    """
    defs, fns, def_tokens = [], [], set()
    ctx: list[tuple[bool, tuple[str, ...]]] = [(file_is_test, ())] * len(tokens)
    stack = [{"kind": "file", "test": file_is_test, "impl": None, "trait": None, "mods": ()}]
    header: list[tuple[str, str]] = []
    pending_test = False
    i, n = 0, len(tokens)
    while i < n:
        kind, text = tokens[i]
        top = stack[-1]
        ctx[i] = (top["test"], top["mods"])
        if text == "#" and i + 1 < n and (
            tokens[i + 1][1] == "[" or (tokens[i + 1][1] == "!" and i + 2 < n and tokens[i + 2][1] == "[")
        ):
            inner = tokens[i + 1][1] == "!"
            j = start = i + (3 if inner else 2)
            depth = 1
            while j < n and depth:
                depth += {"[": 1, "]": -1}.get(tokens[j][1], 0)
                j += 1
            for k in range(i, j):
                ctx[k] = ctx[i]
            if is_test_attribute([word for _, word in tokens[start:j - 1]]):
                if inner:
                    top["test"] = True
                else:
                    pending_test = True
            i = j
            continue
        if top["kind"] in ITEM_SCOPES:
            if text in (";", "}"):
                if text == "}" and len(stack) > 1:
                    stack.pop()
                header, pending_test = [], False
                i += 1
                continue
            if text == "{":
                words = [word for _, word in header]
                scope = dict(top, kind="block", test=top["test"] or pending_test)
                keyword = position = None
                for position, word in enumerate(words):
                    if word in OPENING:
                        keyword = word
                        break
                    if word == "=":
                        break
                if keyword == "mod" and position + 1 < len(words):
                    scope.update(kind="mod", mods=top["mods"] + (words[position + 1],), impl=None, trait=None)
                elif keyword == "impl":
                    self_type, trait = impl_self_type(header[position + 1:])
                    scope.update(kind="impl", impl=self_type, trait=trait)
                elif keyword == "trait":
                    scope.update(kind="trait", impl=None, trait=words[position + 1] if position + 1 < len(words) else None)
                elif keyword == "fn":
                    scope["kind"] = "fn"
                elif keyword == "extern" and "fn" not in words:
                    scope["kind"] = "mod"
                stack.append(scope)
                header, pending_test = [], False
                i += 1
                continue
            header.append(tokens[i])
            if kind == "id" and text in DEFINING and i + 1 < n:
                j = i + 1
                if text == "macro_rules" and tokens[j][1] == "!":
                    j += 1
                if text == "static" and tokens[j][1] == "mut":
                    j += 1
                if j < n and tokens[j][0] == "id" and tokens[j][1] not in KEYWORDS:
                    words = [word for _, word in header[:-1]]
                    test = top["test"] or pending_test
                    defs.append({
                        "name": tokens[j][1], "kind": text, "test": test, "scope": top["kind"],
                        "public": "pub" in words or top["kind"] == "trait",
                    })
                    def_tokens.add(j)
                    if text == "fn":
                        fns.append({"start": i, "impl": top["impl"], "trait": top["trait"],
                                    "test": test, "scope": top["kind"]})
            i += 1
            continue
        if text == "{":
            stack.append(dict(top, kind="block"))
        elif text == "}":
            if len(stack) > 1:
                stack.pop()
            if stack[-1]["kind"] in ITEM_SCOPES:
                header, pending_test = [], False
        i += 1
    for fn in fns:
        j, depth = fn["start"], 0
        while j < n:
            word = tokens[j][1]
            if word in ("(", "["):
                depth += 1
            elif word in (")", "]"):
                depth -= 1
            elif word in ("{", ";") and depth == 0:
                break
            j += 1
        fn["end"] = j
    return {"ctx": ctx, "defs": defs, "fns": fns, "def_tokens": def_tokens}


# ---------------------------------------------------------------- references


def module_path(path: str) -> tuple[str, ...]:
    if path == "main.rs":
        return ("<main>",)
    parts = path[:-3].split("/")
    return tuple(parts[:-1] if parts[-1] in ("lib", "mod") else parts)


def parse_path_tree(tokens, j, prefix, out):
    """Parse a path or use tree at tokens[j]; append (segments, alias) leaves to out."""
    n = len(tokens)
    segments = list(prefix)
    while j < n:
        kind, text = tokens[j]
        if kind == "id" and text != "as":
            segments.append((text, j))
            j += 1
            if j < n and tokens[j][1] == "::":
                j += 1
                continue
            alias = None
            if j + 1 < n and tokens[j][1] == "as" and tokens[j + 1][0] == "id":
                alias = tokens[j + 1][1]
                j += 2
            out.append((segments, alias))
            return j
        if text == "{":
            j += 1
            while j < n and tokens[j][1] != "}":
                if tokens[j][1] == ",":
                    j += 1
                elif tokens[j][1] == "*":
                    out.append((segments + [("*", j)], None))
                    j += 1
                else:
                    after = parse_path_tree(tokens, j, segments, out)
                    j = after if after > j else j + 1
            return j + 1
        if text == "*":
            out.append((segments + [("*", j)], None))
            return j + 1
        if segments:
            out.append((segments, None))
        return j
    return j


def is_use_start(tokens, i) -> bool:
    return tokens[i][1] == "use" and tokens[i][0] == "id" and (
        i == 0 or tokens[i - 1][1] in (";", "{", "}", "]", "pub", ")", "crate")
    )


def is_camel(word: str) -> bool:
    return word[:1].isupper() and not word.isupper()


def count_whole_app_signatures(tokens, fn) -> set[str]:
    """`&RSpiceApp`/`&mut RSpiceApp` occurrences in a function signature."""
    found = set()
    for k in range(fn["start"], min(fn["end"], len(tokens))):
        if tokens[k][1] != "RSpiceApp":
            continue
        j = k - 1
        while j > fn["start"] and tokens[j][1] == "::" and tokens[j - 1][0] == "id":
            j -= 2
        mutable = tokens[j][1] == "mut"
        j -= mutable
        if tokens[j][1] == "'lt":
            j -= 1
        if tokens[j][1] == "&":
            found.add("&mut RSpiceApp params outside shell" if mutable else "&RSpiceApp params outside shell")
    return found


def measure(source_root: Path, lower_crates: dict[str, Path] | None = None):
    """Measure a source tree. Returns (values, contributors, stats).

    `lower_crates` maps a slice to the src directory of a crate its files moved
    into. Only that crate's item definitions and module names are read, as the
    slice's own; its files are not measured.
    """
    paths = sorted(p.relative_to(source_root).as_posix() for p in source_root.rglob("*.rs"))
    sources = {p: (source_root / p).read_text(encoding="utf-8", errors="replace") for p in paths}
    unassigned = [p for p in paths if slice_of(p) is None]
    if unassigned:
        raise SystemExit("no slice rule matches:\n  " + "\n  ".join(unassigned)
                         + "\nadd a rule to SLICE_RULES in tools/ci/check_app_slices.py")
    owner = {p: slice_of(p) for p in paths}
    is_test = test_files(sources)
    tokens = {p: lex(sources[p]) for p in paths}
    scans = {p: scan(tokens[p], is_test[p]) for p in paths}

    # A moved slice's items and module names are still its own: a bare name or a
    # module segment shared with a moved file never resolves to a feature's item.
    definitions = [(owner[p], scans[p]["defs"]) for p in paths]
    lower_modules = set()
    for slice_name, lower_root in (lower_crates or {}).items():
        lower = {p.relative_to(lower_root).as_posix(): p.read_text(encoding="utf-8", errors="replace")
                 for p in sorted(lower_root.rglob("*.rs"))}
        lower_is_test = test_files(lower)
        definitions += [(slice_name, scan(lex(text), lower_is_test[p])["defs"]) for p, text in lower.items()]
        lower_modules |= {module_path(p) for p in lower}

    module_file = {module_path(p): p for p in paths}
    prefixes = {m[:length] for m in module_file for length in range(len(m) + 1)}
    stems = {m[-1] for m in (*module_file, *lower_modules) if m}

    # Public free items whose name only one slice defines, at any visibility.
    defined_in, public_in, kinds = (collections.defaultdict(set) for _ in range(3))
    for slice_name, defs in definitions:
        for d in defs:
            if d["scope"] in ("file", "mod") and d["kind"] != "mod" and not d["test"]:
                defined_in[d["name"]].add(slice_name)
                kinds[d["name"]].add(d["kind"])
                if d["public"]:
                    public_in[d["name"]].add(slice_name)
    # Names other code also spells as an enum variant (`TransferFailure::Request`,
    # `SheetText::Field`, `SessionKept::WithoutRuns`), as a test-only item of its
    # own (`thread_preparations::record` under `#[cfg(test)]`), or as an item
    # another crate defines and a module re-exports (`crate::state::Point`,
    # `crate::ui::input::consume_keys`, `crate::workbench::design_system::elide_text`,
    # `crate::ui::widgets::segmented`, `persistent::page`), which a bare name
    # cannot tell apart, are left out; `crate::` paths to the item itself are
    # still counted below.
    symbols = {
        name: next(iter(found)) for name, found in public_in.items()
        if len(name) >= 4 and len(defined_in[name]) == 1
        and name not in ("Cell", "Align", "Request", "ResultRun", "Point", "Command", "Field",
                         "Identity", "Action", "DataTable", "GeneratedNetlist",
                         "WithoutRuns", "record", "consume_keys", "elide_text", "segmented", "page")
    }

    # `use` declarations: re-export table, external roots, per-file imports and globs.
    reexports, external_names = {}, collections.defaultdict(set)
    external_roots, globs, imports = set(), collections.defaultdict(list), collections.defaultdict(list)
    for p in paths:
        toks, ctx, here = tokens[p], scans[p]["ctx"], module_path(p)
        i = 0
        while i < len(toks):
            if not is_use_start(toks, i):
                i += 1
                continue
            current = here + ctx[i][1]
            out = []
            j = i + 1
            if j < len(toks) and toks[j][1] == "::":
                j += 1
            end = parse_path_tree(toks, j, [], out)
            for segments, alias in out:
                names = [name for name, _ in segments]
                k, base = 0, None
                if not names:
                    continue
                if names[0] in ("crate", "self", "super"):
                    base, k = ((), 1) if names[0] == "crate" else (current, 0)
                    while k < len(names) and names[k] in ("self", "super"):
                        base = base[:-1] if names[k] == "super" else base
                        k += 1
                elif current + (names[0],) in prefixes:
                    base = current
                rest = names[k:]
                leaf = alias or (rest[-1] if rest else (current[-1] if current else ""))
                if base is None:
                    external_roots.add(names[0])
                    if leaf and leaf != "*":
                        external_names[p].add(leaf)
                        reexports.setdefault((current, leaf), "EXT")
                elif rest and rest[-1] == "*":
                    globs[p].append(base + tuple(rest[:-1]))
                elif rest:
                    if rest[-1] == "self":
                        rest = rest[:-1]
                        leaf = alias or (rest[-1] if rest else "")
                    if rest:
                        reexports.setdefault((current, leaf), base + tuple(rest))
                        imports[p].append((leaf, base + tuple(rest)))
            i = max(end, i + 1)

    def resolve(path: tuple[str, ...], glob: bool = False) -> str | None:
        """The file defining a path, following module files and re-exports.

        None when the path leaves the crate; a glob instead stops at the
        module that imported the external name.
        """
        for _ in range(12):
            length = next(length for length in range(len(path), -1, -1)
                          if path[:length] in module_file or length == 0)
            # A `use` inside an inline module (`mod lifecycle { pub use ..; }`) is
            # keyed by that module's path, which no file defines.
            scope = next((inner for inner in range(len(path) - 1, length, -1)
                          if (path[:inner], path[inner]) in reexports), length)
            target = reexports.get((path[:scope], path[scope])) if scope < len(path) else None
            if target == "EXT" and not glob:
                return None
            if target not in (None, "EXT") and target != path[:scope + 1]:
                path = target + path[scope + 1:]
                continue
            break
        return module_file.get(path[:length])

    imported = collections.defaultdict(dict)
    for p in paths:
        for leaf, target in imports[p]:
            defining = resolve(target)
            if defining is not None:
                imported[p][leaf] = owner[defining]
    glob_slices, via_glob = collections.defaultdict(set), collections.defaultdict(dict)
    for round_ in range(3):
        for p in paths:
            for target in globs[p]:
                defining = resolve(target, glob=True)
                if defining is None:
                    continue
                glob_slices[p].add(owner[defining])
                for name, slice_name in list(imported[defining].items()) + list(via_glob[defining].items()):
                    via_glob[p].setdefault(name, slice_name)
                if round_:
                    glob_slices[p] |= glob_slices[defining]

    edges = collections.Counter()
    edge_files = collections.defaultdict(collections.Counter)
    for p in paths:
        toks, sc, source = tokens[p], scans[p], owner[p]
        ctx, here, n = sc["ctx"], module_path(p), len(toks)
        counted = set()

        def record(target: str, i: int) -> None:
            if not ctx[i][0]:
                edges[(source, target)] += 1
                edge_files[(source, target)][p] += 1

        in_use = False
        for i, (kind, word) in enumerate(toks):
            if is_use_start(toks, i):
                in_use = True
            elif word == ";":
                in_use = False
            if in_use or kind != "id" or word not in symbols or i in sc["def_tokens"] or word in external_names[p]:
                continue
            target = symbols[word]
            before = toks[i - 1][1] if i else ""
            after = toks[i + 1][1] if i + 1 < n else ""
            if before == "." or after == ":":
                continue
            if before == "::":
                j = i
                while j >= 2 and toks[j - 1][1] == "::" and toks[j - 2][0] == "id":
                    j -= 2
                root = toks[j][1]
                if root not in ("crate", "super", "self", "Self") and root in external_roots \
                        and here + (root,) not in prefixes:
                    continue
            else:
                if word in imported[p]:
                    target = imported[p][word]
                elif word in via_glob[p]:
                    target = via_glob[p][word]
                elif target not in glob_slices[p]:
                    continue
            if kinds[word] <= {"macro_rules"}:
                if after != "!":
                    continue
            elif not is_camel(word) and not word.isupper():
                if after == "::" and word in stems:
                    continue
                if not (after in ("(", "::") or before == "::"):
                    continue
                if before == "::" and i >= 2 and is_camel(toks[i - 2][1]):
                    continue
            if target != source:
                counted.add(i)
                record(target, i)

        i = 0
        while i < n:
            kind, word = toks[i]
            if not (kind == "id" and word in ("crate", "super", "self") and i + 1 < n and toks[i + 1][1] == "::"
                    and (i == 0 or toks[i - 1][1] != "::")):
                i += 1
                continue
            if i >= 2 and toks[i - 1][1] == "in" and toks[i - 2][1] == "(":
                while i < n and toks[i][1] != ")":
                    i += 1
                continue
            out = []
            end = parse_path_tree(toks, i, [], out)
            for segments, _ in out:
                names = [name for name, _ in segments]
                k, base = 0, here + ctx[i][1]
                if names[0] == "crate":
                    k, base = 1, ()
                while k < len(names) and names[k] in ("self", "super"):
                    base = base[:-1] if names[k] == "super" else base
                    k += 1
                rest = tuple(name for name in names[k:] if name != "*")
                if not rest:
                    continue
                defining = resolve(base + rest)
                if defining is None or owner[defining] == source or segments[-1][1] in counted:
                    continue
                record(owner[defining], i)
            i = max(end, i + 1)

    values = {group: {} for group in GROUPS}
    contributors = {}
    for (source, target), count in edges.items():
        group = edge_group(source, target)
        if group:
            key = f"{source}->{target}"
            values[group][key] = count
            contributors[(group, key)] = edge_files[(source, target)]

    whole = collections.defaultdict(collections.Counter)
    for p in paths:
        for fn in scans[p]["fns"]:
            if fn["test"]:
                continue
            inherent = fn["scope"] == "impl" and fn["trait"] is None
            if inherent and fn["impl"] == "RSpiceApp" and owner[p] != SHELL:
                whole["impl RSpiceApp methods outside shell"][p] += 1
            if inherent and fn["impl"] == "AppState" and owner[p] != SESSION:
                whole["impl AppState methods outside session"][p] += 1
            if owner[p] != SHELL:
                for key in count_whole_app_signatures(tokens[p], fn):
                    whole[key][p] += 1
    for key, files in whole.items():
        values["whole-app"][key] = sum(files.values())
        contributors[("whole-app", key)] = files

    stats = {"files": len(paths), "production": sum(not t for t in is_test.values()),
             "tokens": sum(map(len, tokens.values()))}
    return values, contributors, stats


# ---------------------------------------------------------------- ratchet


def top_files(counter, limit: int = 3) -> str:
    ranked = sorted(counter.items(), key=lambda item: (-item[1], item[0]))[:limit]
    return ", ".join(f"{path} {count}" for path, count in ranked)


def compare(values, ceilings, contributors=None) -> list[str]:
    """Every difference between the measurement and the ceilings, as messages."""
    contributors = contributors or {}
    problems = [f"unknown ceiling group {group!r}" for group in sorted(set(ceilings) - set(GROUPS))]
    for group in GROUPS:
        measured, allowed = values.get(group, {}), ceilings.get(group, {})
        for key in sorted(set(measured) | set(allowed)):
            count, ceiling = measured.get(key, 0), allowed.get(key)
            label = f"{group} {key}"
            files = contributors.get((group, key))
            where = f"; top files: {top_files(files)}" if files else ""
            if ceiling is None:
                problems.append(f"{label}: {count} reference(s) with no ceiling entry; this coupling is new{where}")
            elif count == 0:
                problems.append(f"{label}: measures 0; delete its ceiling entry")
            elif count > ceiling:
                problems.append(f"{label}: {count} exceeds the ceiling {ceiling} by {count - ceiling}{where}")
            elif count < ceiling:
                problems.append(f"{label}: {count} is below the ceiling {ceiling}; lower the ceiling to {count}")
    return problems


def summary(values, stats, seconds: float) -> list[str]:
    lines = [f"rspice-app slices: {stats['files']} files ({stats['production']} production), "
             f"{stats['tokens']:,} tokens, {seconds:.1f} s"]
    for group in GROUPS[:2]:
        edges = values[group]
        lines.append(f"  {group}: {sum(edges.values())} references in {len(edges)} edges")
    for key, count in sorted(values["whole-app"].items()):
        lines.append(f"  {key}: {count}")
    return lines


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--write-ceilings", action="store_true",
                        help=f"rewrite {CEILINGS.name} from this measurement")
    args = parser.parse_args(argv)
    started = time.perf_counter()
    if not (SOURCE / "lib.rs").is_file():
        print(f"{SOURCE / 'lib.rs'} is missing; the ratchet would measure nothing")
        return 1
    values, contributors, stats = measure(SOURCE, {slice_name: root for slice_name, root in LOWER_CRATES.items()
                                                   if root.is_dir()})
    print("\n".join(summary(values, stats, time.perf_counter() - started)))
    if args.write_ceilings:
        ordered = {group: dict(sorted(values[group].items())) for group in GROUPS}
        with open(CEILINGS, "w", encoding="utf-8", newline="\n") as handle:
            handle.write(json.dumps(ordered, indent=2) + "\n")
        print(f"wrote {CEILINGS.relative_to(ROOT).as_posix()}")
        return 0
    problems = compare(values, json.loads(CEILINGS.read_text(encoding="utf-8")), contributors)
    if problems:
        print(f"\n{len(problems)} slice ceiling problem(s):")
        print("\n".join(f"  {problem}" for problem in problems))
        print(
            "\nA ceiling records coupling still to be removed and is never raised to make room. "
            "When a count falls, lower its ceiling in the same change; --write-ceilings rewrites "
            f"{CEILINGS.name}, whose diff must then only lower or delete entries."
        )
        return 1
    print(f"every measurement equals its ceiling ({sum(map(len, values.values()))} entries)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
