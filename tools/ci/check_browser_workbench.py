#!/usr/bin/env python3
"""Qualify the real browser workbench, its durable recovery workflow, and the
results a saved project keeps in the browser's own storage across a reload.

Serve the normal web entry and production worker, with the UI built using
`browser-qualification`. The optional observer exposes rendered controls only.
All edits use WebDriver input. Retain this run's output directory on failure.
"""

from __future__ import annotations

import argparse
from contextlib import contextmanager
import copy
import csv
import hashlib
import io
import json
import math
import re
from pathlib import Path
import struct
import time
import uuid

from browser_workbench import WorkbenchBrowser, controls, unsaved, wait_for

REVIEW_TITLE = "Browser recovery qualification"
REVIEW_BODY = "Verify that this review survives recovery export."
RESOLUTION = "Resolution verified in the browser."
CLOCK_FAULTS = ("negative", "nonfinite", "infinite", "zero", "fractional")


@contextmanager
def wall_clock(browser, value):
    """Fault the environment clock without changing application state or input."""
    browser.record_input("wall_clock_override", value=value)
    browser.script("""
        if (window.__rspiceOriginalClock) throw new Error('Clock override already active');
        const values = {negative: -1, nonfinite: NaN, infinite: Infinity, zero: 0, fractional: 1.5};
        const value = typeof arguments[0] === 'number' ? arguments[0] : values[arguments[0]];
        if (value === undefined) throw new Error('Unknown clock override');
        window.__rspiceOriginalClock = Date.now;
        Date.now = () => value;
    """, value)
    try:
        yield
    finally:
        browser.script("Date.now = window.__rspiceOriginalClock; delete window.__rspiceOriginalClock;")
        browser.record_input("wall_clock_restored")


def reports(rendered, text):
    # A Console row carries its line as its label; a status line, as its value.
    text = text.lower()
    return any(text in control["label"].lower() or text in control["value"].lower()
               for control in rendered)


def verify_clock_rejection(snapshot, operation, draft=None):
    rendered = controls(snapshot)
    if not reports(rendered, f"{operation} could not be timestamped"):
        raise AssertionError("The operation did not report its timestamp failure")
    if draft is not None and not any(control["value"] == draft and
                                    control["role"] in ("textInput", "multilineTextInput")
                                    for control in rendered):
        raise AssertionError("The failed timestamp operation lost or changed its draft")


def reject_review_clock(browser, action, draft, name):
    for fault in CLOCK_FAULTS:
        with wall_clock(browser, fault):
            browser.click(action, "button")
            verify_clock_rejection(browser.capture(f"{name}-clock-{fault}"), "Review update", draft)


def clear_console(browser):
    # Clear is named for what it does, and is off while the log is empty.
    if any(control["label"] == "Clear the session log" for control in controls(browser.snapshot())):
        browser.click("Clear the session log", "button")


def reject_checkpoint_clock(browser, name):
    # A refused manual checkpoint is a Console error and a notice, not a toast.
    browser.click("Expand console", "button")
    before = checkpoint_records(browser)
    for fault in CLOCK_FAULTS:
        clear_console(browser)
        with wall_clock(browser, fault):
            browser.click("Create checkpoint", "button")
            verify_clock_rejection(browser.capture(f"{name}-clock-{fault}"), "Project checkpoint")
            if checkpoint_records(browser) != before:
                raise AssertionError("A failed timestamp operation changed durable checkpoint records")
    browser.click("Collapse console", "button")


def verify_age(browser, name, now_ms):
    for value, expected, suffix in ((now_ms - 86_400_000, "clock skew", "rollback"),
                                    (now_ms + 86_400_000, "yesterday", "forward"),
                                    ("nonfinite", "time unavailable", "unavailable")):
        with wall_clock(browser, value):
            snapshot = browser.capture(f"{name}-age-{suffix}")
            if not any(control["value"] == expected for control in controls(snapshot)):
                raise AssertionError(f"{name} did not report {expected!r} after a clock adjustment")


def verify_checkpoint_time(browser, manifest, now_ms):
    """A checkpoint row states when the checkpoint was made, as a date and time
    no clock fault moves or hides, and then its age, which a fault makes
    unknown or moves the way the clock moved."""
    stamp = time.strftime("%Y-%m-%d %H:%M UTC", time.gmtime(manifest["created_unix_ms"] // 1000))
    for value, age, suffix in ((now_ms - 86_400_000, "time unavailable", "rollback"),
                               (now_ms + 86_400_000, "yesterday", "forward"),
                               ("nonfinite", "time unavailable", "unavailable")):
        row = f"Manual checkpoint, {stamp} · {age} · revision {manifest['project_revision']},"
        with wall_clock(browser, value):
            snapshot = browser.capture(f"checkpoint-age-{suffix}")
            if not any(control["label"].startswith(row) for control in controls(snapshot)):
                raise AssertionError(f"The checkpoint row did not read {row!r} after a clock adjustment")


def verify_revision_time(browser, record, note):
    """The history states when a revision was made as a date and time that no
    clock fault moves or hides, in the row that names the revision's note."""
    stamp = time.strftime("%Y-%m-%d %H:%M UTC", time.gmtime(record["created_unix_ms"] // 1000))
    now_ms = time.time_ns() // 1_000_000
    for value, suffix in ((now_ms - 86_400_000, "rollback"),
                          (now_ms + 86_400_000, "forward"),
                          ("nonfinite", "unavailable")):
        with wall_clock(browser, value):
            snapshot = browser.capture(f"revision-time-{suffix}")
            if not any(f"Time: {stamp}" in control["label"] and f"Change: {note}" in control["label"]
                       for control in controls(snapshot)):
                raise AssertionError(f"The revision did not state {stamp!r} after a clock adjustment")


def canonical_projects(browser):
    result = browser.call("POST", "/execute/async", {"args": [], "script": """
        const done = arguments[arguments.length - 1];
        (async () => {
            const root = await navigator.storage.getDirectory();
            let directory;
            try { directory = await root.getDirectoryHandle('rspice-projects', {create: false}); }
            catch (error) { if (error.name === 'NotFoundError') return {}; throw error; }
            const files = {};
            for await (const [name, handle] of directory.entries()) {
                if (handle.kind === 'file' && name.endsWith('.rspiceproj')) {
                    files[name] = await (await handle.getFile()).text();
                }
            }
            return files;
        })().then(files => done({files}), error => done({error: String(error)}));
    """})
    if "error" in result:
        raise AssertionError("Canonical project observation failed: " + result["error"])
    return result["files"]


def validated_save_clock(browser, review_intervals):
    browser.click("Close", "button")
    browser.click("Schematic", "button")
    # At the fresh document's 100% zoom, construct a closed 5 V / 1 kohm
    # circuit through normal placement and routing. Ground's terminal is 10
    # units above its origin; the resistor and source terminals are 20 apart
    # from their origins. The validation dialog must confirm this actual circuit.
    for command, offset in (("Place ground", (120, 100)),
                            ("Place resistor", (140, 90)),
                            ("Place voltage source", (120, 70))):
        choose_command(browser, command)
        browser.click("Schematic canvas", "canvas", offset=offset)
        browser.keys("\ue00c")
    browser.click("Draw wire (W)", "button")
    for offset in ((120, 50), (160, 50), (160, 90)):
        browser.click("Schematic canvas", "canvas", offset=offset)
    browser.keys("\ue007")
    browser.snapshot()  # Observe route completion before leaving the tool.
    browser.keys("\ue00c")

    note = "Validated browser clock revision"
    # A landed save closes its form and says so only in the session log, so
    # the log is emptied while nothing modal covers the Console.
    browser.click("Expand console", "button")
    clear_console(browser)
    browser.click("Collapse console", "button")
    choose_command(browser, "Check and save", keyboard=((note, ()),))
    snapshot = browser.capture("validated-save-preconditions")
    if not any(control["label"] == "Checks" and control["value"] == "No blockers or advisories"
               for control in controls(snapshot)):
        raise AssertionError("The authored circuit did not pass schematic validation")
    before = canonical_projects(browser)
    if before:
        raise AssertionError("A first-save fixture already contains a canonical project")
    for fault in CLOCK_FAULTS:
        with wall_clock(browser, fault):
            browser.click("Save", "button")
            verify_clock_rejection(browser.capture(f"validated-save-clock-{fault}"),
                                   "validated revision", note)
            if canonical_projects(browser) != before:
                raise AssertionError("A failed revision clock published a canonical project")

    # Exercise the real OPFS fallback without invoking an OS-owned save picker
    # in headless Chrome. No file handles, permissions or application state are
    # supplied: the normal save action creates and verifies its own project.
    browser.record_input("file_picker_unavailable")
    browser.script("window.__rspiceOriginalSavePicker = window.showSaveFilePicker; window.showSaveFilePicker = undefined;")
    started_ms = time.time_ns() // 1_000_000
    try:
        browser.click("Save", "button")
        # A refused save keeps the form open on its note; a landed one closes it.
        wait_for(lambda: not any(control["label"] == "Revision note" and control["role"] == "textInput"
                                 for control in controls(browser.snapshot())),
                 "the canonical validated save")
    finally:
        browser.script("window.showSaveFilePicker = window.__rspiceOriginalSavePicker; delete window.__rspiceOriginalSavePicker;")
        browser.record_input("file_picker_restored")
    finished_ms = time.time_ns() // 1_000_000
    # The completed save states its verdict as a session-log line. It is read,
    # not inferred from the form closing: an observable that disappears is a
    # failure, never a pass.
    browser.click("Expand console", "button")
    wait_for(lambda: reports(controls(browser.snapshot()), "Saved a validated revision"),
             "the validated save's session-log record")
    browser.capture("validated-save-complete")
    browser.click("Collapse console", "button")
    files = canonical_projects(browser)
    if len(files) != 1:
        raise AssertionError("Validated save did not publish exactly one canonical project")
    filename, raw = next(iter(files.items()))
    project = json.loads(raw)
    verify_review(project, review_intervals)
    records = project["workspace"]["schematic_buffers"]["user/top/schematic"]["validated_revisions"]["records"]
    saved = [record for record in records if record["kind"] == "validated_save"]
    if len(saved) != 1 or saved[0]["revision_note"] != note:
        raise AssertionError("The canonical project lost or duplicated its validated revision")
    if any(not started_ms <= record["created_unix_ms"] <= finished_ms for record in records):
        raise AssertionError("The canonical project retained a fabricated revision timestamp")
    (browser.output / "validated-project.rspiceproj").write_bytes(raw.encode("utf-8"))
    choose_command(browser, "Revision history")
    verify_revision_time(browser, saved[0], note)
    browser.click("Close", "button")
    browser.call("POST", "/refresh", {})
    wait_until_ready(browser)
    browser.click("Schematic canvas", "canvas")
    choose_command(browser, "Revision history")
    reloaded = browser.capture("validated-history-reloaded")
    if not any(f"Change: {note}" in control["label"] for control in controls(reloaded)):
        raise AssertionError("Reload did not restore the validated revision history")
    if canonical_projects(browser) != files:
        raise AssertionError("Reload changed the canonical project bytes")
    return {"clock_failures": {fault: "passed" for fault in CLOCK_FAULTS},
            "save_and_reload": "passed", "opfs_file": filename,
            "sha256": hashlib.sha256(raw.encode()).hexdigest(),
            "interval_ms": [started_ms, finished_ms]}


def model_validation_clock(browser):
    browser.click("Close", "button")
    choose_command(browser, "Corners & sections")
    browser.click("Expand console", "button")
    clear_console(browser)
    before_save = canonical_projects(browser)
    started_ms = time.time_ns() // 1_000_000
    browser.click("Validate bindings", "button")
    # A validation that worked says nothing on the page; the durable receipt
    # read back below is its evidence.
    browser.capture("model-bindings-validated")
    finished_ms = time.time_ns() // 1_000_000
    # Receipt-only changes must enable Save all even when no library changed.
    choose_command(browser, "Save all")
    browser.capture("model-validation-save-requested")

    def persisted():
        files = canonical_projects(browser)
        if files == before_save:
            return None
        if len(files) != 1:
            raise AssertionError("Model validation changed the canonical project identity")
        project = json.loads(next(iter(files.values())))
        receipt = project["execution_context"].get("model_validation_receipt")
        return (files, receipt) if receipt else None

    files, receipt = wait_for(persisted, "the persisted model-validation receipt")
    wait_for(lambda: not unsaved(browser.snapshot()),
             "the accepted model-validation save")
    if (not started_ms <= receipt["validated_at_unix_ms"] <= finished_ms
            or receipt["platform"] != "browser-wasm32"):
        raise AssertionError("Model validation retained an incorrect timestamp or runtime")
    for fault in CLOCK_FAULTS:
        clear_console(browser)
        with wall_clock(browser, fault):
            browser.click("Validate bindings", "button")
            snapshot = browser.capture(f"model-validation-clock-{fault}")
            if not reports(controls(snapshot), "system clock cannot timestamp model validation"):
                raise AssertionError("Model validation did not report its clock failure")
        if unsaved(browser.snapshot()):
            raise AssertionError("A failed model validation dirtied the saved project")
        if canonical_projects(browser) != files:
            raise AssertionError("A failed validation replaced the canonical model receipt")

    # Also snapshot the working project: unchanged saved bytes alone could hide
    # an invalid receipt left in memory by the failed operation.
    choose_command(browser, "Recovery")
    before = checkpoint_records(browser)
    started_ms = time.time_ns() // 1_000_000
    browser.click("Create checkpoint", "button")
    records = wait_for_checkpoint_publication(browser, before, "the checkpoint after failed model validation")
    _, _, project = verify_checkpoint(records, started_ms, time.time_ns() // 1_000_000)
    if project["execution_context"].get("model_validation_receipt") != receipt:
        raise AssertionError("Failed validation changed the working model receipt")
    browser.capture("model-validation-retained")
    choose_command(browser, "Corners & sections")
    retry_started_ms = time.time_ns() // 1_000_000
    browser.click("Validate bindings", "button")
    retry_finished_ms = time.time_ns() // 1_000_000
    before_save = files
    # Normal Save must include the active model catalog's evidence as well.
    choose_command(browser, "Save")
    files, retried_receipt = wait_for(persisted, "the model receipt saved after clock recovery")
    if (retried_receipt == receipt or not retry_started_ms <=
            retried_receipt["validated_at_unix_ms"] <= retry_finished_ms):
        raise AssertionError("Healthy model validation did not publish a fresh receipt")
    expected = json.loads(next(iter(before_save.values())))
    expected["execution_context"]["model_validation_receipt"] = retried_receipt
    if json.loads(next(iter(files.values()))) != expected:
        raise AssertionError("Saving a new model receipt changed unrelated project content")
    wait_for(lambda: not unsaved(browser.snapshot()),
             "the accepted active-model save")
    (browser.output / "model-validation-project.rspiceproj").write_bytes(next(iter(files.values())).encode("utf-8"))
    browser.capture("model-validation-retried")
    return {"clock_failures": {fault: "passed" for fault in CLOCK_FAULTS},
            "retained_receipt": receipt, "interval_ms": [started_ms, finished_ms],
            "retried_receipt": retried_receipt,
            "retry_interval_ms": [retry_started_ms, retry_finished_ms]}


def provider_decision_clock(browser):
    # Feed the app's actual folder picker through WebDriver. The normal import
    # worker reads, parses and authenticates these source files.
    browser.cdp("Page.setInterceptFileChooserDialog", {"enabled": True})
    try:
        for name, kp in (("first", "1e-3"), ("second", "2e-3")):
            folder = browser.output / "fixtures" / name
            folder.mkdir(parents=True)
            fixture = folder / (name + ".lib")
            fixture.write_bytes(f".model shared NMOS (LEVEL=1 KP={kp})\n".encode())
            browser.record_input("select_model_source_folder", path=str(folder),
                                 sha256=hashlib.sha256(fixture.read_bytes()).hexdigest())
            browser.click("Import section map", "button")
            elements = wait_for(lambda: browser.call("POST", "/elements", {
                "using": "css selector", "value": 'input[aria-label="Select model source folder"]',
            }), "the model source folder input")
            if len(elements) != 1:
                raise AssertionError("Expected one model source folder picker")
            identity = elements[0]["element-6066-11e4-a52e-4f735466cecf"]
            browser.call("POST", f"/element/{identity}/value", {"text": str(folder), "value": list(str(folder))})
            wait_for(lambda: any("Imported" in text and name in text
                                 for c in controls(browser.snapshot())
                                 for text in (c["label"], c["value"])), "the imported " + name + " source")
            browser.capture("provider-imported-" + name)
    finally:
        browser.cdp("Page.setInterceptFileChooserDialog", {"enabled": False})
    choose_command(browser, "Save all")
    wait_for(lambda: not unsaved(browser.snapshot()),
             "the saved model sources")
    files = canonical_projects(browser)
    if len(files) != 1:
        raise AssertionError("Model import changed the canonical project identity")
    decisions = []
    for index, (provider, save) in enumerate((("second", "Save"), ("first", "Save all"))):
        choose_command(browser, "Include graph")
        browser.click("", "textInput")
        browser.keys("a", modifiers=("\ue009",))
        browser.keys("shared")
        candidates = [c for c in controls(browser.snapshot()) if c["label"].startswith("shared ")]
        if len(candidates) != 1:
            raise AssertionError("Expected one contested shared model row")
        browser.click(candidates[0]["label"], candidates[0]["role"])
        if index and not any(c["role"] == "radioButton" and c["label"] == decisions[-1]["provider_library"]
                             and c["toggled"] == "true" for c in controls(browser.snapshot())):
            raise AssertionError("Reload did not restore the accepted model provider")
        browser.click(provider, "radioButton")
        draft = f"Use the reviewed {provider} characterization source.\nRetain this audit reason."
        browser.click("", "multilineTextInput")
        browser.keys(draft)
        if not any(c["role"] == "multilineTextInput" and c["value"] == draft
                   for c in controls(browser.capture(f"provider-{index}-draft"))):
            raise AssertionError("The provider audit editor did not receive the complete multiline draft")
        for fault in CLOCK_FAULTS:
            with wall_clock(browser, fault):
                browser.click("Publish provider decision", "button")
                snapshot = browser.capture(f"provider-{index}-clock-{fault}")
                rendered = controls(snapshot)
                if not reports(rendered, "system clock cannot timestamp provider decision"):
                    raise AssertionError("Provider publication did not report its clock failure")
                if not any(c["role"] == "multilineTextInput" and c["value"] == draft for c in rendered):
                    raise AssertionError("Provider publication lost its editable audit reason")
                if not any(c["role"] == "radioButton" and c["label"] == provider
                           and c["toggled"] == "true" for c in rendered):
                    raise AssertionError("Provider publication lost its selected source")
            if canonical_projects(browser) != files:
                raise AssertionError("Failed provider publication changed the saved project")
            if unsaved(browser.snapshot()):
                raise AssertionError("Failed provider publication dirtied the saved project")
        started_ms = time.time_ns() // 1_000_000
        browser.click("Publish provider decision", "button")
        finished_ms = time.time_ns() // 1_000_000
        if any(c["label"] == "Publish provider decision" for c in controls(browser.snapshot())):
            raise AssertionError("Successful provider publication did not complete the dialog")
        choose_command(browser, save)

        def persisted():
            current = canonical_projects(browser)
            if current == files:
                return None
            if set(current) != set(files):
                raise AssertionError("Provider save changed the canonical project identity")
            return current

        files = wait_for(persisted, "the saved model provider decision")
        wait_for(lambda: not unsaved(browser.snapshot()),
                 "the accepted provider save")
        records = json.loads(next(iter(files.values())))["execution_context"]["model_resolution_records"]
        if len(records) != 1:
            raise AssertionError("Expected exactly one accepted provider decision")
        record = records[0]
        if (record["provider_library"] != provider or record["audit_reason"] != draft
                or record["normalized_name"] != "shared" or record["consumer_scope"] != "primitive_model"
                or not started_ms <= record["created_at_unix_ms"] <= finished_ms):
            raise AssertionError("The provider save did not retain its exact decision and timestamp")
        decisions.append(record)
        browser.capture(f"provider-{index}-saved")
        browser.click("Schematic", "button")
        browser.call("POST", "/refresh", {})
        wait_until_ready(browser)
        browser.click("Schematic canvas", "canvas")
        if canonical_projects(browser) != files:
            raise AssertionError("Reload changed the accepted provider project")
    (browser.output / "model-provider-project.rspiceproj").write_bytes(next(iter(files.values())).encode("utf-8"))
    execution = run_with_saved_provider(browser, decisions[-1])
    return {"clock_failures": {fault: "passed" for fault in CLOCK_FAULTS},
            "save_and_reload": "passed", "decisions": decisions, "execution": execution}


def run_with_saved_provider(browser, decision):
    # Run the circuit authored above after restoring the model decision,
    # including its multiline audit reason, through the normal project loader.
    browser.click("Run active simulation plan at TT · 27 °C (Ctrl+Enter)", "button")
    # Run leaves the reader on the schematic and opens Results when the run
    # completes there; how fast this deck finishes decides which of the two
    # the next frame shows. The Simulation workspace states the plan's newest
    # run either way, so read the outcome there rather than race the hop.
    choose_command(browser, "Open simulation workspace")
    wait_for(lambda: any(c["value"] == "Run 1 finished"
                        for c in controls(browser.snapshot())), "the saved-provider simulation", 120)
    browser.capture("provider-run-completed")
    choose_command(browser, "Recovery")
    before = checkpoint_records(browser)
    started_ms = time.time_ns() // 1_000_000
    browser.click("Create checkpoint", "button")
    records = wait_for_checkpoint_publication(browser, before, "the saved-provider result checkpoint")
    _, raw, project = verify_checkpoint(records, started_ms, time.time_ns() // 1_000_000)
    if project["execution_context"]["model_resolution_records"] != [decision]:
        raise AssertionError("Simulation changed the restored provider decision")
    runs = project["simulation_results"]["runs"]
    if (len(runs) != 1 or not runs[0]["success"] or runs[0]["lifecycle"] != "completed"
            or len(runs[0]["analyses"]) != 1):
        raise AssertionError("The restored provider did not produce one successful analysis")
    analysis = runs[0]["analyses"][0]
    waveforms = analysis["waveforms"]
    if not analysis["success"] or analysis.get("error_message") or not waveforms:
        raise AssertionError("The saved-provider analysis has no successful transient samples")
    # The checkpoint names the run's samples in its results store; their values
    # are read back from the reopened project once it has been saved and reloaded.
    for waveform in waveforms:
        x, y = waveform["x"], waveform["y"]
        if not stored_window_pair(x, y):
            raise AssertionError("The saved-provider transient samples are incomplete or invalid")
    if not any(w["name"].upper().startswith("V(") for w in waveforms):
        raise AssertionError("The saved-provider simulation did not retain the authored output")
    (browser.output / "model-provider-results.rspiceproj").write_bytes(raw)
    return {"worker_run": "passed", "waveforms": {w["name"]: w["x"]["rows"] for w in waveforms}}


def stored_window_pair(x, y):
    """Whether a waveform's axis and values are windows of the same stored segment: the axis its
    first column, the values another, both holding the same two or more rows."""
    fields = {"segment", "column", "column_rows", "first", "rows"}
    if not all(isinstance(values, dict) and fields <= set(values) for values in (x, y)):
        return False
    return (x["segment"] == y["segment"] and x["column"] == 0 and y["column"] > 0
            and x["rows"] >= 2 and x["rows"] == y["rows"] and x["first"] == y["first"]
            and x["column_rows"] == y["column_rows"] >= x["first"] + x["rows"])


def create_and_resolve_review(browser):
    choose_command(browser, "Review comments", input_burst=True)
    # The click and immediate editing input share one WebDriver request.
    browser.click("New comment", "button", keyboard=(("a", ("\ue009",)), (REVIEW_TITLE, ())))
    browser.click("Place", "button")
    browser.click("Schematic canvas", "canvas")
    browser.keys("\ue00c")
    choose_command(browser, "Review comments", keyboard=((REVIEW_BODY, ()),))
    reject_review_clock(browser, "Post reply", REVIEW_BODY, "review-post")
    posted_start = time.time_ns() // 1_000_000
    browser.click("Post reply", "button")
    browser.capture("review-posted")
    posted_end = time.time_ns() // 1_000_000
    browser.click("", "multilineTextInput")
    browser.keys(RESOLUTION)
    reject_review_clock(browser, "Resolve with note", RESOLUTION, "review-resolve")
    resolved_start = time.time_ns() // 1_000_000
    browser.click("Resolve with note", "button")
    resolved_end = time.time_ns() // 1_000_000
    browser.click("Comment status filter", "comboBox")
    browser.click("All comments · 1", "listBoxOption")
    browser.capture("review-resolved")
    verify_age(browser, "review", resolved_end)
    browser.click("Close", "button")
    return [(posted_start, posted_end), (resolved_start, resolved_end)]


def verify_review(project, intervals):
    notes = project["workspace"]["schematic_buffers"]["user/top/schematic"]["design_notes"]
    if len(notes) != 1 or notes[0]["kind"] != "review_note" or notes[0]["text"] != REVIEW_TITLE:
        raise AssertionError("The authored review note was not retained in the schematic")
    review = notes[0]["review"]
    if review["state"] != "resolved" or review["resolution_note"] != RESOLUTION:
        raise AssertionError("The review resolution was not retained")
    messages = review["messages"]
    if [message["body"] for message in messages] != [REVIEW_BODY, RESOLUTION]:
        raise AssertionError("Posted review messages changed or disappeared")
    for message, (start, end) in zip(messages, intervals, strict=True):
        if not start <= message["created_unix_ms"] <= end or not message["author"]:
            raise AssertionError(f"Review timestamp or author was not retained: {message}")


def checkpoint_records(browser):
    """Read existing checkpoint records using an independent readonly transaction."""
    result = browser.call("POST", "/execute/async", {"args": [], "script": """
        const done = arguments[arguments.length - 1];
        (async () => {
            const name = 'rspice-project-bindings';
            if (!(await indexedDB.databases()).some(db => db.name === name)) {
                done({records: {}});
                return;
            }
            const open = indexedDB.open(name);
            open.onerror = () => done({error: String(open.error)});
            open.onupgradeneeded = () => open.transaction.abort();
            open.onsuccess = () => {
                const db = open.result;
                const records = {};
                const transaction = db.transaction('canonical-file-handles', 'readonly');
                const cursor = transaction.objectStore('canonical-file-handles').openCursor();
                cursor.onsuccess = () => {
                    const row = cursor.result;
                    if (!row) return;
                    if (String(row.key).startsWith('rspice.project-recovery.v1.')) {
                        records[row.key] = typeof row.value === 'string'
                            ? row.value : Array.from(new Uint8Array(row.value));
                    }
                    row.continue();
                };
                transaction.oncomplete = () => { db.close(); done({records}); };
                transaction.onabort = () => {
                    db.close(); done({error: String(transaction.error)});
                };
            };
        })().catch(error => done({error: String(error)}));
    """})
    if "error" in result:
        raise AssertionError("Checkpoint observation failed: " + result["error"])
    return result["records"]


def wait_for_checkpoint_publication(browser, before, description, timeout=30):
    """Wait for the commit manifest; the snapshot is published in an earlier transaction."""
    def published():
        added = {key: value for key, value in checkpoint_records(browser).items() if key not in before}
        return added if any(key.endswith(".manifest") for key in added) else None

    return wait_for(published, description, timeout)


def verify_checkpoint(records, started_ms, finished_ms):
    manifest, raw, project = checkpoint_pair(records)
    if manifest["schema_version"] != 2 or manifest["reason"] != "manual" or manifest["pinned"]:
        raise AssertionError(f"Unexpected checkpoint identity: {manifest}")
    if not started_ms <= manifest["created_unix_ms"] <= finished_ms:
        raise AssertionError(f"Checkpoint wall time is outside the observed creation interval: {manifest}")
    return manifest, raw, project


def checkpoint_pair(records):
    """The one checkpoint `records` hold, read back and checked against its manifest."""
    manifests = [(key, json.loads(value)) for key, value in records.items()
                 if key.endswith(".manifest")]
    if len(manifests) != 1 or len(records) != 2:
        raise AssertionError(f"Expected one fresh checkpoint pair; got {list(records)}")
    key, manifest = manifests[0]
    snapshot_key = key.removesuffix(".manifest") + ".snapshot"
    if snapshot_key not in records:
        raise AssertionError("Checkpoint manifest has no matching snapshot")
    raw = bytes(records[snapshot_key])
    if len(raw) != manifest["snapshot_byte_len"]:
        raise AssertionError("Checkpoint length differs from its durable manifest")
    if hashlib.sha256(raw).hexdigest() != manifest["snapshot_digest"]:
        raise AssertionError("Checkpoint digest differs from its durable manifest")
    project = json.loads(raw)
    descriptor = project["workspace"]["project"]
    if (descriptor["id"], descriptor["name"], descriptor["revision"]) != (
            manifest["project_id"], manifest["project_name"], manifest["project_revision"]):
        raise AssertionError("Checkpoint manifest belongs to a different project identity")
    return manifest, raw, project


def verify_recovery_copy(project, recovered, filename):
    descriptor = recovered["workspace"]["project"]
    identity = uuid.UUID(descriptor["id"])
    if not identity.int or descriptor["id"] == project["workspace"]["project"]["id"]:
        raise AssertionError("Recovery must publish an independent project identity")
    if descriptor["path"] != filename:
        raise AssertionError("Recovery path does not identify the exported copy")
    expected = copy.deepcopy(project)
    expected["workspace"]["project"].update(id=descriptor["id"], path=filename)
    if recovered != expected:
        raise AssertionError("Recovery changed content beyond the new identity and destination")


def restore_checkpoint(browser, records, project, review_intervals):
    """Restore asks in the checkpoint's row before it changes anything, and a
    declined restore leaves every checkpoint as it was. A restore that goes
    ahead keeps the project as it is now as a "Before restore" checkpoint
    first, changes no other checkpoint, and says what it restored."""
    question = "Restore “Manual checkpoint” from"
    browser.click("Restore Manual checkpoint into the project", "button")
    if not reports(controls(browser.capture("checkpoint-restore-asked")), question):
        raise AssertionError("Restore went ahead without asking")
    browser.click("Cancel", "button")
    if reports(controls(browser.snapshot()), question) or checkpoint_records(browser) != records:
        raise AssertionError("A declined restore was not dismissed or changed the checkpoints")
    browser.click("Restore Manual checkpoint into the project", "button")
    browser.click("Restore Manual checkpoint", "button")
    wait_for_checkpoint_publication(browser, records, "the before-restore checkpoint")
    wait_for(lambda: reports(controls(browser.snapshot()), "Restored “Manual checkpoint” from"),
             "the restore's report")
    browser.capture("checkpoint-restored")
    after = checkpoint_records(browser)
    if any(after.get(key) != value for key, value in records.items()):
        raise AssertionError("Restoring changed the checkpoint it restored")
    added = {key: value for key, value in after.items() if key not in records}
    manifest, _, kept = checkpoint_pair(added)
    if manifest["reason"] != "before-restore":
        raise AssertionError(f"The restore kept the project under another reason: {manifest}")
    if kept["workspace"]["project"]["id"] != project["workspace"]["project"]["id"]:
        raise AssertionError("The before-restore checkpoint holds another project")
    verify_review(kept, review_intervals)


def choose_command(browser, label, *, input_burst=False, keyboard=()):
    if input_burst:
        browser.key_sequence((("k", ("\ue009",)), ("wrong", ()), ("a", ("\ue009",)), (label, ())))
        snapshot = browser.capture("palette-input-burst")
        search = [control for control in controls(snapshot)
                  if control["label"] == "Command search" and control["role"] == "comboBox"]
        if len(search) != 1 or search[0]["value"] != label:
            raise AssertionError(f"Opening palette input was lost or routed incorrectly: {search}")
    else:
        browser.keys("k", modifiers=("\ue009",))
        wait_for(lambda: any(control["label"] == "Command search" and control["role"] == "comboBox"
                            for control in controls(browser.snapshot())), "the command palette search field")
        browser.click("Command search", "comboBox")
        browser.keys("a", modifiers=("\ue009",))
        browser.keys(label)
    wait_for(lambda: any(control["label"] == label and control["role"] == "listBoxOption"
                        for control in controls(browser.snapshot())), f"the {label!r} command")
    for control in controls(browser.snapshot()):
        if (control["label"] == label and control["role"] == "listBoxOption" and
                control["description"].startswith("Unavailable:")):
            raise AssertionError(f"{label}: {control['description']}")
    if label == "Save all":
        browser.capture("save-all-command")
    browser.click(label, "listBoxOption", keyboard=keyboard)


def configuration_opening_input(browser):
    """Typing straight after New configuration is clicked lands in its Name,
    and Escape leaves the project without the configuration."""
    name, new = "Browser configuration draft", "New configuration\u2026"
    choose_command(browser, "Configuration sets")
    wait_for(lambda: any(control["label"] == new and control["role"] == "button"
                        for control in controls(browser.snapshot())), "the Configuration page")
    browser.click(new, "button", keyboard=(("a", ("\ue009",)), (name, ())))

    def names(snapshot):
        return [control for control in controls(snapshot)
                if control["role"] == "textInput" and control["label"] == "Name"]

    fields = names(browser.capture("configuration-name"))
    if len(fields) != 1 or fields[0]["value"] != name:
        raise AssertionError(f"Configuration opening input was lost or misdirected: {fields}")
    browser.keys("\ue00c")
    wait_for(lambda: not names(browser.snapshot()), "New configuration to close on Escape")
    if any(name in control["label"] for control in controls(browser.capture("configuration-cancelled"))):
        raise AssertionError("Escape created the configuration it cancelled")
    # The review that follows places its note on the schematic.
    choose_command(browser, "Open design workspace")
    wait_for(lambda: any(control["label"] == "Schematic canvas" and control["role"] == "canvas"
                        for control in controls(browser.snapshot())), "the schematic canvas")


SAVE_PAGE = "Save, streaming & retention policy"
KEPT = "kept in this browser's storage"
HELD_ELSEWHERE = "another tab holds this project's results store"
UNREAD = "cannot be read"
BROWSER_FILE_OPERATION = "Browser file operation"


def results_storage(rendered):
    """The Save page's Results storage statement: what the page holds in memory, and whether the
    project's results store in the browser's own storage keeps it."""
    statements = {text for control in rendered for text in (control["label"], control["value"])
                  if " in this page's memory · " in text}
    if len(statements) > 1:
        raise AssertionError(f"The Save page states its results storage twice: {sorted(statements)}")
    return next(iter(statements), None)


def segments_named(project):
    """Every results store segment the saved project's runs read, by digest."""
    named = set()
    for run in project["simulation_results"]["runs"]:
        for analysis in run["analyses"]:
            for waveform in analysis["waveforms"]:
                for values in (waveform["x"], waveform["y"]):
                    if not isinstance(values, dict) or "segment" not in values:
                        raise AssertionError(f"{waveform['name']} is not a stored window: {values}")
                    named.add(values["segment"])
    return named


def results_store_files(browser, store):
    result = browser.call("POST", "/execute/async", {"args": [store], "script": """
        const done = arguments[arguments.length - 1];
        const store = arguments[0];
        (async () => {
            const root = await navigator.storage.getDirectory();
            let directory;
            try {
                directory = await (await root.getDirectoryHandle('rspice-results'))
                    .getDirectoryHandle(store);
            } catch (error) { if (error.name === 'NotFoundError') return {}; throw error; }
            const files = {};
            const walk = async (folder, prefix) => {
                for await (const [name, handle] of folder.entries()) {
                    if (handle.kind === 'directory') await walk(handle, prefix + name + '/');
                    else files[prefix + name] = true;
                }
            };
            await walk(directory, '');
            return files;
        })().then(files => done({files}), error => done({error: String(error)}));
    """})
    if "error" in result:
        raise AssertionError("Results store observation failed: " + result["error"])
    return sorted(result["files"])


def stated_storage(browser, wanted, description, timeout=120):
    """The Save page's statement once it says `wanted`. A page just loaded may still be opening
    its project's store, so the page is opened again until it states the storage."""
    deadline = time.monotonic() + timeout

    def statement():
        text = results_storage(controls(browser.snapshot()))
        return text if text and wanted in text else None

    while True:
        try:
            choose_command(browser, SAVE_PAGE)
            return wait_for(statement, description, min(20, max(1, deadline - time.monotonic())))
        except AssertionError as error:
            if not str(error).startswith("Timed out waiting for") or time.monotonic() >= deadline:
                raise
            browser.capture("results-storage-retry")


def results_survive_reload(browser):
    """The run the saved project holds keeps its samples in the browser's own storage: saved,
    reloaded and reopened, the project's run reads them, and nothing says they cannot be read."""
    before = stated_storage(browser, KEPT, "the run's results kept in this browser's storage")
    browser.capture("results-kept")
    held(before)
    # The document tab that states whether the project is saved shows on the schematic.
    browser.click("Schematic", "button")
    choose_command(browser, "Save all")
    wait_for(lambda: not unsaved(browser.snapshot()), "the project saved with its run", 60)
    files = canonical_projects(browser)
    if len(files) != 1:
        raise AssertionError(f"Expected one saved browser project, found {sorted(files)}")
    project = json.loads(next(iter(files.values())))
    named = segments_named(project)
    if not named:
        raise AssertionError("The saved project names no results store segment")
    store = project["workspace"]["project"]["id"]
    kept = results_store_files(browser, store)
    if "manifest.rsm" not in kept or not any(name.startswith("runs/") for name in kept):
        raise AssertionError(f"The project's results store keeps no segment: {kept}")
    browser.click("Expand console", "button")
    clear_console(browser)
    browser.click("Collapse console", "button")
    browser.call("POST", "/refresh", {})
    wait_until_ready(browser)
    browser.click("Schematic canvas", "canvas")
    palette_stays_open(browser, "once the saved project is read back")
    after = stated_storage(browser, KEPT, "the reopened run's results read from this browser's storage")
    if after.split(" · ")[-1] != before.split(" · ")[-1]:
        raise AssertionError(f"The reopened store states other results than it kept: {before!r}, then {after!r}")
    held(after)
    browser.click("Expand console", "button")
    reopened = browser.capture("results-reopened")
    browser.click("Collapse console", "button")
    if reports(controls(reopened), UNREAD):
        raise AssertionError("The reopened project says its run's results cannot be read")
    if json.loads(next(iter(canonical_projects(browser).values()))) != project:
        raise AssertionError("Reopening changed the saved project")
    samples = verify_exported_samples(exported_dataset(browser))
    return {"store": store, "segments": sorted(named), "store_files": kept,
            "before_reload": before, "after_reload": after, "exported_samples": samples}


def held(statement):
    """The Save page's statement counts the run the page reads: its figure for the page's memory
    is not nothing while the page holds the run's samples."""
    if statement.startswith("0 B of "):
        raise AssertionError(f"The page states it holds nothing of the run it reads: {statement!r}")


def palette_stays_open(browser, when):
    """The command palette opened once the page's project is read back stays open while its
    results store opens: nothing the store's opening brings closes what the reader opened."""
    browser.keys("k", modifiers=("\ue009",))

    def search():
        return any(control["label"] == "Command search" and control["role"] == "comboBox"
                   for control in controls(browser.snapshot()))

    wait_for(search, "the command palette search field")
    browser.capture("palette-opened")
    deadline = time.monotonic() + 30
    while time.monotonic() < deadline:
        if not search():
            browser.capture("palette-closed")
            raise AssertionError(f"The command palette opened {when} closed by itself")
        time.sleep(0.25)
    browser.keys("\ue00c")
    wait_for(lambda: not search(), "the command palette closed")


def clear_result_history(browser, delete=None):
    """Delete the runs of the result history, through Results > History > Clear result history…,
    the one place it is offered. `delete`, when given, is the confirmation the reader must see."""
    browser.click("Results", "menuItem")
    wait_for(lambda: any(c["label"] == "History" and c["role"] == "menuItem"
                         for c in controls(browser.snapshot())), "the Results menu")
    browser.click("History", "menuItem")
    items = wait_for(lambda: [c for c in controls(browser.snapshot())
                              if c["label"].startswith("Clear result history")],
                     "the Results menu's History")
    click_control(browser, items[0])
    deletes = wait_for(lambda: [c for c in controls(browser.snapshot())
                                if re.fullmatch(r"Delete \d+ runs?", c["label"]) and c["role"] == "button"],
                       "the clear result history confirmation")
    if delete is not None and [c["label"] for c in deletes] != [delete]:
        raise AssertionError(f"Clearing the history offers {[c['label'] for c in deletes]}, not {delete!r}")
    click_control(browser, deletes[0])
    deleted = deletes[0]["label"].replace("Delete", "Deleted", 1)
    wait_for(lambda: reports(controls(browser.snapshot()), deleted), "the cleared result history")


def restored_checkpoint_reads_its_run(browser, store):
    """A checkpoint keeps the samples of the run it holds: with the run cleared from the project,
    the project saved and the page reloaded, restoring the checkpoint reads the run's samples
    back from the browser's own storage, and nothing says they cannot be read."""
    clear_result_history(browser, "Delete 1 run")
    browser.click("Schematic", "button")
    choose_command(browser, "Save all")
    wait_for(lambda: not unsaved(browser.snapshot()), "the project saved without its run", 60)
    files = canonical_projects(browser)
    if segments_named(json.loads(next(iter(files.values())))):
        raise AssertionError("The project saved without its run still names a segment")
    browser.click("Expand console", "button")
    clear_console(browser)
    browser.click("Collapse console", "button")
    browser.call("POST", "/refresh", {})
    wait_until_ready(browser)
    browser.click("Schematic canvas", "canvas")
    kept = results_store_files(browser, store)
    if not any(name.startswith("runs/") for name in kept):
        raise AssertionError(f"The save dropped the segment a checkpoint reads: {kept}")
    choose_command(browser, "Recovery")
    label = "Restore Manual checkpoint into the project"
    # Checkpoints are listed newest first: the top one holds the run.
    restores = wait_for(lambda: sorted((c for c in controls(browser.snapshot())
                                        if c["label"] == label and c["role"] == "button"),
                                       key=lambda c: c["center"][1]), "the checkpoint holding the run")
    click_control(browser, restores[0])
    browser.click("Restore Manual checkpoint", "button")
    wait_for(lambda: reports(controls(browser.snapshot()), "Restored “Manual checkpoint” from"),
             "the restore of the checkpoint holding the run")
    browser.click("Expand console", "button")
    restored = browser.capture("checkpoint-run-restored")
    browser.click("Collapse console", "button")
    if reports(controls(restored), UNREAD):
        raise AssertionError("The restored checkpoint says its run's results cannot be read")
    samples = verify_exported_samples(exported_dataset(browser))
    return {"store_files": kept, "exported_samples": samples}


def exported_dataset(browser):
    """The reopened run's samples, as Export result dataset… writes them to a CSV download."""
    # A dataset exported before was read already; the one this export writes is the only one.
    for earlier in browser.downloads.glob("*.csv"):
        earlier.replace(earlier.with_name(earlier.name + ".read"))
    choose_command(browser, "Open results workspace")
    # The export writes what the waveform sheet shows, so every signal of the run is plotted.
    plots = wait_for(lambda: [c for c in controls(browser.snapshot())
                              if c["role"] == "checkBox" and c["label"].startswith("Plot ")],
                     "the reopened run's signals")
    for plot in plots:
        if plot["toggled"] != "true":
            browser.click(plot["label"], "checkBox")
    browser.capture("results-plotted")
    # The Results toolbar has an Export… of its own; the form's is the one the form brings.
    toolbar = {c["id"] for c in controls(browser.snapshot()) if c["label"] == "Export…"}
    browser.click("Export result dataset…", "button")
    rendered = wait_for(lambda: [c for c in controls(browser.snapshot())
                                 if c["role"] == "dialog" and c["label"] == "Export result dataset"]
                        and controls(browser.snapshot()), "the export result dataset form")
    every = [c for c in rendered if c["label"].startswith("All (")]
    if len(every) > 1:
        raise AssertionError(f"Expected at most one choice of every signal to export; got {every}")
    if every:
        browser.click(every[0]["label"], every[0]["role"])
    browser.capture("results-export-form")
    primary = [c for c in controls(browser.snapshot()) if c["label"] == "Export…"
               and c["role"] == "button" and c["id"] not in toolbar]
    if len(primary) != 1:
        raise AssertionError(f"Expected the form to bring one Export… button; got {primary}")
    click_control(browser, primary[0])

    def download():
        files = list(browser.downloads.glob("*.csv"))
        if len(files) > 1:
            raise AssertionError(f"Unexpected extra dataset downloads: {files}")
        return files[0] if files and not list(browser.downloads.glob("*.crdownload")) else None

    return wait_for(download, "the exported result dataset", 60).read_text(encoding="utf-8")


def click_control(browser, control):
    """Click one rendered control where it is drawn, for a button that shares its name with one
    outside the form it belongs to."""
    canvas = browser.script("""
        const r = document.getElementById('rspice_canvas').getBoundingClientRect();
        return {x: r.x, y: r.y, ratio: devicePixelRatio};
    """)
    x, y = (round(canvas[axis] + control["center"][i] / canvas["ratio"])
            for i, axis in enumerate(("x", "y")))
    browser.record_input("click", label=control["label"], role=control["role"], node=control["id"],
                         x=x, y=y, keyboard=[])
    pointer = [{"type": "pointerMove", "duration": 0, "origin": "viewport", "x": x, "y": y},
               {"type": "pointerDown", "button": 0}, {"type": "pause", "duration": 60},
               {"type": "pointerUp", "button": 0}]
    browser.call("POST", "/actions", {"actions": [
        {"type": "pointer", "id": "mouse", "parameters": {"pointerType": "mouse"}, "actions": pointer}]})
    browser.snapshot()


def verify_exported_samples(text):
    """The exported transient runs to 1 ms on a rising time axis and holds the authored 5 V
    output at every sample. A row whose first cell starts with # describes the table's layout."""
    rows = [row for row in csv.reader(io.StringIO(text)) if row and not row[0].startswith("#")]
    if len(rows) < 3:
        raise AssertionError(f"The exported dataset holds fewer than two samples: {rows[:3]}")
    header, samples = rows[0], [[float(value) for value in row] for row in rows[1:]]
    if any(len(row) != len(header) for row in samples):
        raise AssertionError("The exported dataset has rows of different widths")
    time_axis = [row[0] for row in samples]
    if (not all(math.isfinite(v) for v in time_axis) or any(b < a for a, b in zip(time_axis, time_axis[1:]))
            or not math.isclose(time_axis[-1], 0.001, rel_tol=0, abs_tol=1e-12)):
        raise AssertionError("The exported transient samples are incomplete or invalid")
    if not any(name.strip().upper().startswith("V(")
               and all(math.isclose(row[column], 5.0, rel_tol=0, abs_tol=1e-6) for row in samples)
               for column, name in enumerate(header) if column):
        raise AssertionError("The reopened run did not retain the authored 5 V output")
    return {"signals": header[1:], "rows": len(samples)}


def second_tab_refused(browser):
    """A second tab that opens the project finds its results store held by the first, and keeps
    nothing of its own in it."""
    first = browser.call("GET", "/window")
    second = browser.call("POST", "/window/new", {"type": "tab"})["handle"]
    browser.call("POST", "/window", {"handle": second})
    try:
        browser.navigate(browser.origin)
        wait_until_ready(browser)
        browser.capture("second-tab-opened")
        # A click anywhere on the workbench gives it the keyboard; the schematic rail button
        # is drawn whichever workspace the tab opens on.
        browser.click("Schematic", "button")
        statement = stated_storage(browser, HELD_ELSEWHERE, "the second tab refused the results store")
        browser.capture("second-tab-refused")
    finally:
        browser.call("DELETE", "/window")
        browser.call("POST", "/window", {"handle": first})
    browser.click("Schematic", "button")
    stated_storage(browser, KEPT, "the first tab still holding the results store")
    return {"second_tab": statement}


QUOTA_WRITE = """
    const done = arguments[arguments.length - 1];
    (async () => {
        const worker = new Worker(window.__RSPICE_SIM_WORKER_URL, {type: 'module'});
        const pending = new Map();
        let next = 1;
        worker.onmessage = ({data}) => {
            const waiting = pending.get(data.id);
            if (!waiting) return;
            pending.delete(data.id);
            if (data.type === 'results-store-result') waiting.resolve(data);
            else waiting.reject(new Error(String(data.error)));
        };
        const ask = (request, buffers = []) => new Promise((resolve, reject) => {
            const id = next++;
            pending.set(id, {resolve, reject});
            worker.postMessage({type: 'results-store', id, request: JSON.stringify(request), buffers},
                               buffers.map(view => view.buffer));
        }).then(answer => JSON.parse(answer.response));
        const store = 'quota-' + crypto.randomUUID();
        try {
            const opened = await ask({op: 'open', store});
            if (opened.op !== 'opened' || opened.standing.kind !== 'held') throw new Error(JSON.stringify(opened));
            const signals = 3, rows = (8 << 20) / ((signals + 1) * 8);
            const begun = await ask({op: 'begin', digest: '0'.repeat(64), schema: {
                x: ['time', 's'], analysis: 'tran', sweep: [], seed: null, options: [],
                signals: [0, 1, 2].map(signal => ({
                    name: 'v(n' + signal + ')', unit: 'V', complex: false, quantity: '', path: ''}))
            }});
            if (begun.op !== 'begun') throw new Error(JSON.stringify(begun));
            let state = 7, chunks = 0, full = false;
            const random = () => {
                state ^= state << 13; state ^= state >>> 17; state ^= state << 5;
                return (state >>> 0) / 4294967296;
            };
            while (!full && chunks < 64) {
                const x = new Float64Array(rows), values = new Float64Array(rows * signals);
                for (let row = 0; row < rows; row++) x[row] = (chunks * rows + row) * 1e-9;
                for (let i = 0; i < values.length; i++) values[i] = random();
                const taken = await ask({op: 'rows', write: begun.write}, [x, values]);
                if (taken.op !== 'taken') throw new Error(JSON.stringify(taken));
                full = taken.full;
                chunks++;
            }
            const finished = await ask({op: 'finish', write: begun.write});
            if (finished.op !== 'finished') throw new Error(JSON.stringify(finished));
            await ask({op: 'close'});
            await ask({op: 'open', store});
            const checked = await ask({op: 'check', digest: finished.record.digest});
            return {chunks, full: finished.full, record: finished.record, checked};
        } finally {
            await ask({op: 'close'}).catch(() => {});
            worker.terminate();
            const root = await navigator.storage.getDirectory();
            await (await root.getDirectoryHandle('rspice-results'))
                .removeEntry(store, {recursive: true}).catch(() => {});
        }
    })().then(done, error => done({error: String(error)}));
"""


def verify_quota_stop(result, quota):
    if "error" in result:
        raise AssertionError("The quota write failed: " + result["error"])
    if not result["full"] or result["record"]["rows"] <= 0:
        raise AssertionError(f"The quota did not stop the write on rows it kept: {result}")
    if result["checked"].get("op") != "checked" or result["checked"]["len"] <= 0:
        raise AssertionError(f"The rows the full store kept did not read back: {result}")
    if result["checked"]["len"] > quota:
        raise AssertionError(f"The full store kept more than the quota allows: {result}")
    return {"quota_bytes": quota, "rows_kept": result["record"]["rows"],
            "bytes_kept": result["checked"]["len"], "chunks_sent": result["chunks"]}


FULL = "this browser's storage for RSpice is full"


def quota_stop_cuts_a_page_run(browser):
    """A run whose samples outgrow the storage the browser gives RSpice keeps the rows the store
    took: the Console says the storage is full and how many of the run's points it kept. The run
    is one the store does not keep already, since the store keeps a run's samples once, and it
    writes megabytes, more than the room the quota leaves past what the store keeps for closing a
    segment and the grain the browser grants room in."""
    estimate = browser.call("POST", "/execute/async", {"args": [], "script": """
        const done = arguments[arguments.length - 1];
        navigator.storage.estimate().then(done, error => done({error: String(error)}));
    """})
    if "error" in estimate:
        raise AssertionError("The storage estimate failed: " + estimate["error"])
    quota = int(estimate["usage"]) + (3 << 20)
    choose_command(browser, "Open simulation workspace")
    browser.click("Analyses, 1 / 1", "button")

    def field(label):
        found = [control for control in controls(browser.snapshot())
                 if control["label"] == label and control["role"] == "textInput"]
        return found[0]["value"] if len(found) == 1 else None

    wait_for(lambda: field("Stop time") is not None, "the transient's Stop time")
    for label, value in (("Stop time", "10m"), ("Max step", "10n")):
        browser.click(label, "textInput", keyboard=(("a", ("\ue009",)), (value, ()), ("\ue007", ())))
        wait_for(lambda: field(label) == value, f"the transient's {label} set to {value}")
    browser.click("Expand console", "button")
    clear_console(browser)
    browser.click("Collapse console", "button")
    browser.cdp("Storage.overrideQuotaForOrigin", {"origin": browser.origin, "quotaSize": quota})
    try:
        browser.click("Run active simulation plan at TT · 27 °C (Ctrl+Enter)", "button")
        wait_for(lambda: reports(controls(browser.snapshot()), "results storage full"),
                 "the run stopped by the full storage", 300)
        # The finished run opens on Results, whose console starts collapsed; the stop's finding
        # is on its Problems tab.
        browser.click("Expand console", "button")
        problems = wait_for(lambda: [c for c in controls(browser.snapshot())
                                     if c["role"] == "tab" and c["label"].startswith("Problems")],
                            "the console's Problems tab")
        click_control(browser, problems[0])
        rendered = wait_for(lambda: [c for c in controls(browser.snapshot()) if FULL in c["label"]],
                            "the full storage's finding", 60)
        stopped = browser.capture("quota-stop-page-run")
        browser.click("Collapse console", "button")
    finally:
        browser.cdp("Storage.overrideQuotaForOrigin", {"origin": browser.origin})
    line = rendered[0]["label"]
    kept = re.search(r"; (\d+) points kept \(", line)
    if not kept:
        raise AssertionError(f"The full storage's line does not say how many points it kept: {line!r}")
    if not 0 < int(kept.group(1)) < 1_000_000:
        raise AssertionError(f"The full storage kept none or all of the run's points: {line!r}")
    # The run cut to the rows the store kept is whole: it passes its check and the project saves.
    if reports(controls(stopped), ", corrupted") or reports(controls(stopped), "Damaged result"):
        raise AssertionError("The run cut to the rows the full store kept fails its check")
    browser.click("Schematic", "button")
    choose_command(browser, "Save all")
    wait_for(lambda: not unsaved(browser.snapshot()), "the project saved with the cut run", 60)
    return {"quota_bytes": quota, "finding": line, "points_kept": int(kept.group(1))}


NOT_IN_BROWSER = "its samples are not kept in this browser"


def quota_refusal_keeps_nothing(browser):
    """A run the store has no room to begin is not kept, and says so: the run states that its
    results are not kept and that none of its points are, and it passes its check. Saved once the
    storage has room again, the project keeps the run: opened again, the run reads its points, and
    is neither called damaged nor said to be missing from this browser."""
    estimate = browser.call("POST", "/execute/async", {"args": [], "script": """
        const done = arguments[arguments.length - 1];
        navigator.storage.estimate().then(done, error => done({error: String(error)}));
    """})
    if "error" in estimate:
        raise AssertionError("The storage estimate failed: " + estimate["error"])
    # Less than the store keeps past a segment for closing it, so the segment cannot begin.
    quota = int(estimate["usage"]) + (64 << 10)
    choose_command(browser, "Open simulation workspace")
    browser.click("Analyses, 1 / 1", "button")

    def field(label):
        found = [control for control in controls(browser.snapshot())
                 if control["label"] == label and control["role"] == "textInput"]
        return found[0]["value"] if len(found) == 1 else None

    wait_for(lambda: field("Stop time") is not None, "the transient's Stop time")
    # A run the store does not keep already, small enough to finish at once.
    for label, value in (("Stop time", "3m"), ("Max step", "auto")):
        browser.click(label, "textInput", keyboard=(("a", ("\ue009",)), (value, ()), ("\ue007", ())))
        wait_for(lambda: field(label) == value, f"the transient's {label} set to {value}")
    browser.cdp("Storage.overrideQuotaForOrigin", {"origin": browser.origin, "quotaSize": quota})
    try:
        browser.click("Run active simulation plan at TT · 27 °C (Ctrl+Enter)", "button")
        wait_for(lambda: reports(controls(browser.snapshot()), "results not kept"),
                 "the run the full storage did not keep", 120)
        browser.click("Expand console", "button")
        problems = wait_for(lambda: [c for c in controls(browser.snapshot())
                                     if c["role"] == "tab" and c["label"].startswith("Problems")],
                            "the console's Problems tab")
        click_control(browser, problems[0])
        rendered = wait_for(lambda: [c for c in controls(browser.snapshot())
                                     if "Not kept: " + FULL in c["label"]],
                            "the finding of the run not kept", 60)
        refused = browser.capture("quota-refusal-not-kept")
        browser.click("Collapse console", "button")
    finally:
        browser.cdp("Storage.overrideQuotaForOrigin", {"origin": browser.origin})
    line = rendered[0]["label"]
    if "; 0 points kept" not in line:
        raise AssertionError(f"The run not kept does not say no point is kept: {line!r}")
    if reports(controls(refused), ", corrupted") or reports(controls(refused), "Damaged result"):
        raise AssertionError("The run not kept fails its check")
    browser.click("Schematic", "button")
    choose_command(browser, "Save all")
    wait_for(lambda: not unsaved(browser.snapshot()), "the project saved with the run not kept", 60)
    browser.call("POST", "/refresh", {})
    wait_until_ready(browser)
    browser.click("Schematic canvas", "canvas")
    choose_command(browser, "Open results workspace")

    def points(shown):
        return [c for c in controls(shown) if c["value"].endswith(" pts") and c["value"] != "0 pts"]

    wait_for(lambda: points(browser.snapshot()), "the reopened run reading the points the save kept", 60)
    browser.click("Expand console", "button")
    problems = wait_for(lambda: [c for c in controls(browser.snapshot())
                                 if c["role"] == "tab" and c["label"].startswith("Problems")],
                        "the reopened console's Problems tab")
    click_control(browser, problems[0])
    reopened = browser.capture("quota-refusal-reopened")
    browser.click("Collapse console", "button")
    for refusal in (", corrupted", "Damaged result", NOT_IN_BROWSER):
        if reports(controls(reopened), refusal):
            raise AssertionError(f"The reopened run the save kept reports {refusal!r}")
    return {"quota_bytes": quota, "finding": line, "reopened": points(reopened)[0]["value"]}


IMPORT_WORKER = "import-worker.js"
IMPORTED_RAWFILE = "browser-import.raw"
IMPORTED_ROWS = 400_000


def write_rawfile(path):
    """A binary rawfile of one transient plot at `path`, large enough that its rows reach the
    store in several reads and requests. Answers its columns, by name, in the file's order."""
    columns = {
        "time": [index * 2.5e-9 for index in range(IMPORTED_ROWS)],
        "V(out)": [3.3 * math.sin(index * 1e-3) for index in range(IMPORTED_ROWS)],
        "I(R1)": [index * 1e-6 - 0.2 for index in range(IMPORTED_ROWS)],
    }
    header = (f"Title: browser import\nDate: today\nPlotname: Transient Analysis\nFlags: real\n"
              f"No. Variables: {len(columns)}\nNo. Points: {IMPORTED_ROWS}\nVariables:\n"
              "\t0\ttime\ttime\n\t1\tV(out)\tvoltage\n\t2\tI(R1)\tcurrent\nBinary:\n")
    flat = [value for row in zip(*columns.values()) for value in row]
    path.write_bytes(header.encode("ascii") + struct.pack(f"<{len(flat)}d", *flat))
    return columns


def import_workers(browser):
    """What the page's import workers did: each one's store requests by kind, its answer and
    whether the page let it go."""
    return browser.script("return window.__rspiceImportWorkers || null;")


def rawfile_import_streams(browser):
    """A binary rawfile import streams in an import worker of its own: the page starts it when
    the import starts, relays the rows it reads to the results store, and lets it go when the
    import ends. Saved and reopened, the imported run reads back every sample the file holds."""
    resources = browser.script(
        "return performance.getEntriesByType('resource').map(entry => entry.name);")
    if any(IMPORT_WORKER in name for name in resources):
        raise AssertionError("The page loaded the import worker before any import started")
    # The imported run is the only one, so the dataset exported after the reload is its.
    clear_result_history(browser)
    columns = write_rawfile(browser.output / IMPORTED_RAWFILE)
    # Each import worker the page starts is recorded: its store requests, answer and release.
    browser.script("""
        window.__rspiceImportWorkers = [];
        const Native = window.Worker;
        window.Worker = class extends Native {
            constructor(url, options) {
                super(url, options);
                if (!String(url).includes('import-worker.js')) return;
                const record = {url: String(url), requests: {}, outcome: null, terminated: false};
                window.__rspiceImportWorkers.push(record);
                this.addEventListener('message', event => {
                    const data = event.data || {};
                    if (data.type === 'store') {
                        const op = JSON.parse(data.request).op;
                        record.requests[op] = (record.requests[op] || 0) + 1;
                    } else if (data.type === 'imported') {
                        record.outcome = JSON.parse(data.outcome).kind;
                    } else if (data.type === 'failed') {
                        record.outcome = 'failed: ' + data.error;
                    }
                });
                const terminate = this.terminate.bind(this);
                this.terminate = () => { record.terminated = true; terminate(); };
            }
        };
    """)
    # The picker the import opens is answered here, not by a chooser on the desktop.
    browser.cdp("Page.setInterceptFileChooserDialog", {"enabled": True})
    try:
        choose_command(browser, "Open results workspace")
        # The import is offered on File > Import, not in the palette.
        browser.click("File", "menuItem")
        wait_for(lambda: any(c["label"] == "Import" and c["role"] == "menuItem"
                             for c in controls(browser.snapshot())), "the File menu")
        browser.click("Import", "menuItem")
        wait_for(lambda: any(c["label"] == "Result dataset\u2026" and c["role"] == "menuItem"
                             for c in controls(browser.snapshot())), "the File menu's Import")
        browser.click("Result dataset\u2026", "menuItem")
        picker = wait_for(lambda: browser.call("POST", "/elements",
                                               {"using": "css selector", "value": "#rfd-input"}),
                          "the result dataset picker")
        element = next(iter(picker[0].values()))
        browser.call("POST", f"/element/{element}/value",
                     {"text": str((browser.output / IMPORTED_RAWFILE).resolve())})
        for step in ("Map signals", "Validate import", "Import"):
            wait_for(lambda: any(c["label"] == step and c["role"] == "button"
                                 for c in controls(browser.snapshot())),
                     f"the import review's {step}", 120)
            browser.click(step, "button")
    finally:
        browser.cdp("Page.setInterceptFileChooserDialog", {"enabled": False})
    wait_for(lambda: not any(c["label"] == "Import result dataset" and c["role"] == "dialog"
                             for c in controls(browser.snapshot())), "the import committed", 60)
    browser.capture("rawfile-imported")
    workers = import_workers(browser)
    if not workers or len(workers) != 1:
        raise AssertionError(f"The import ran in {len(workers or [])} import workers: {workers}")
    worker = workers[0]
    if worker["outcome"] != "kept":
        raise AssertionError(f"The import worker did not keep the rawfile: {worker}")
    requests = worker["requests"]
    if requests.get("begin") != 1 or requests.get("finish") != 1 or requests.get("rows", 0) < 2:
        raise AssertionError(f"The import worker did not stream its rows to the store: {requests}")
    if not worker["terminated"]:
        raise AssertionError("The page kept the import worker after the import ended")
    running = [target for target in browser.cdp("Target.getTargets", {})["targetInfos"]
               if IMPORT_WORKER in target.get("url", "")]
    if running:
        raise AssertionError(f"The import worker outlived its import: {running}")
    browser.click("Schematic", "button")
    choose_command(browser, "Save all")
    wait_for(lambda: not unsaved(browser.snapshot()), "the project saved with the imported run", 60)
    project = json.loads(next(iter(canonical_projects(browser).values())))
    if len(segments_named(project)) != 1:
        raise AssertionError("The imported run does not read one results store segment")
    browser.call("POST", "/refresh", {})
    wait_until_ready(browser)
    browser.click("Schematic canvas", "canvas")
    rows = [row for row in csv.reader(io.StringIO(exported_dataset(browser)))
            if row and not row[0].startswith("#")]
    header, samples = rows[0], rows[1:]
    if len(samples) != IMPORTED_ROWS:
        raise AssertionError(f"The reopened import reads {len(samples)} of {IMPORTED_ROWS} rows")
    for column, name in enumerate(header):
        wanted = columns["time"] if column == 0 else next(
            (values for key, values in columns.items() if key.lower() in name.lower()), None)
        if wanted is None:
            raise AssertionError(f"The reopened import exports a column the file lacks: {name!r}")
        read = [float(row[column]) for row in samples]
        if read != wanted:
            first = next(index for index, (a, b) in enumerate(zip(read, wanted)) if a != b)
            raise AssertionError(f"{name} differs from the file at row {first}: "
                                 f"{read[first]!r} for {wanted[first]!r}")
    return {"requests": requests, "signals": header[1:], "rows": len(samples)}


def quota_stop_keeps_rows(browser):
    """Writing past the origin's quota closes the write on the rows it took: the store keeps them,
    and they read back checked against their digest once the store is opened again."""
    estimate = browser.call("POST", "/execute/async", {"args": [], "script": """
        const done = arguments[arguments.length - 1];
        navigator.storage.estimate().then(done, error => done({error: String(error)}));
    """})
    if "error" in estimate:
        raise AssertionError("The storage estimate failed: " + estimate["error"])
    quota = int(estimate["usage"]) + (24 << 20)
    browser.cdp("Storage.overrideQuotaForOrigin", {"origin": browser.origin, "quotaSize": quota})
    try:
        result = browser.call("POST", "/execute/async", {"args": [], "script": QUOTA_WRITE})
    finally:
        browser.cdp("Storage.overrideQuotaForOrigin", {"origin": browser.origin})
    return verify_quota_stop(result, quota)


def wait_until_ready(browser):
    def ready():
        status = browser.script("""
            return {loading: !!document.getElementById('rspice_loading'),
                worker: document.documentElement.getAttribute('data-rspice-wasm-jit-status'),
                qualification: window.__RSPICE_WASM_JIT_CAPABILITY,
                error: window.__RSPICE_SIM_WORKER_ERROR,
                errors: window.__rspiceQualificationErrors};
        """)
        if status.get("error") or status.get("errors") or status["worker"] == "rejected":
            raise AssertionError(f"Workbench startup failed: {status}")
        return not status["loading"] and status["worker"] == "qualified"

    wait_for(ready, "the real workbench and simulation worker", 120)
    # A page that reopens its project holds the keyboard behind a modal until the project's file
    # and the results it names are read back: a key pressed meanwhile reaches no command.
    wait_for(lambda: not any(control["label"] == BROWSER_FILE_OPERATION
                             for control in controls(browser.snapshot())),
             "the reopened project's file read back", 120)


def run(browser):
    wait_until_ready(browser)
    browser.capture("startup")
    browser.click("Schematic canvas", "canvas")
    configuration_opening_input(browser)
    review_intervals = create_and_resolve_review(browser)
    choose_command(browser, "Recovery")
    wait_for(lambda: any(control["label"] == "Create checkpoint"
                        for control in controls(browser.snapshot())), "the recovery workspace")
    if checkpoint_records(browser):
        raise AssertionError("A fresh profile already contains recovery records")
    reject_checkpoint_clock(browser, "empty-checkpoint-store")
    started_ms = time.time_ns() // 1_000_000
    browser.click("Create checkpoint", "button")

    records = wait_for_checkpoint_publication(browser, {}, "durable checkpoint publication")
    (browser.output / "checkpoint-records.json").write_text(json.dumps(records, indent=2), encoding="utf-8")
    manifest, raw, project = verify_checkpoint(records, started_ms, time.time_ns() // 1_000_000)
    verify_review(project, review_intervals)
    browser.capture("checkpoint-created")
    reject_checkpoint_clock(browser, "retained-checkpoint-store")
    verify_checkpoint_time(browser, manifest, time.time_ns() // 1_000_000)
    # Save as new project... is the row's second action; in a browser it
    # downloads the copy.
    browser.click("More actions for Manual checkpoint", "button")
    browser.click("Save as new project…", "button")

    def download():
        files = list(browser.downloads.glob("*.rspiceproj"))
        if len(files) > 1:
            raise AssertionError(f"Unexpected extra recovery downloads: {files}")
        return files[0] if files and not list(browser.downloads.glob("*.crdownload")) else None

    exported = wait_for(download, "the exported recovery copy")
    verify_recovery_copy(project, json.loads(exported.read_bytes()), exported.name)
    if checkpoint_records(browser) != records:
        raise AssertionError("Exporting a recovery copy mutated its durable checkpoint")
    browser.capture("checkpoint-exported")
    restore_checkpoint(browser, records, project, review_intervals)
    choose_command(browser, "Revision history")
    wait_for(lambda: any("Project revision and audit history" in (control["label"], control["value"])
                        for control in controls(browser.snapshot())), "revision history")
    browser.capture("revision-history")
    validated_save = validated_save_clock(browser, review_intervals)
    model_validation = model_validation_clock(browser)
    provider_decision = provider_decision_clock(browser)
    results_reload = results_survive_reload(browser)
    checkpoint_run = restored_checkpoint_reads_its_run(browser, results_reload["store"])
    second_tab = second_tab_refused(browser)
    quota_stop = quota_stop_keeps_rows(browser)
    quota_page_run = quota_stop_cuts_a_page_run(browser)
    quota_refusal = quota_refusal_keeps_nothing(browser)
    rawfile_import = rawfile_import_streams(browser)
    return {"startup": "passed", "checkpoint_export": "passed", "checkpoint_restore": "passed",
            "revision_history": "opened",
            "configuration_opening_input": "passed",
            "review_and_checkpoint_clock_failures": {fault: "passed" for fault in CLOCK_FAULTS},
            "validated_save": validated_save,
            "model_validation": model_validation,
            "provider_decision": provider_decision,
            "results_survive_reload": results_reload, "results_store_second_tab": second_tab,
            "restored_checkpoint_run": checkpoint_run,
            "results_store_quota_stop": quota_stop, "results_store_quota_page_run": quota_page_run, "results_store_quota_refusal": quota_refusal,
            "rawfile_import_worker": rawfile_import,
            "review_post_and_resolution": "passed", "review_intervals_ms": review_intervals,
            "checkpoint": manifest, "export": exported.name, "project_version": project["version"]}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--web-root", type=Path, default=Path("crates/rspice-app/web"))
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--driver")
    parser.add_argument("--browser")
    parser.add_argument("--software-webgpu", action="store_true",
                        help="Use SwiftShader for functional CI, without qualifying physical GPU support")
    args = parser.parse_args()
    inputs = {}
    for name in ("index.html", "simulation-worker.js", "import-worker.js", "wasm-loader.js",
                 "pkg/rspice-ui.js", "pkg/rspice-ui_bg.wasm",
                 "pkg/rspice-ui-worker.js", "pkg/rspice-ui-worker_bg.wasm",
                 "pkg/rspice-ui-import-worker.js", "pkg/rspice-ui-import-worker_bg.wasm"):
        data = (args.web_root / name).read_bytes()
        inputs[name] = {"sha256": hashlib.sha256(data).hexdigest(), "bytes": len(data)}
    with WorkbenchBrowser(args.web_root, args.output, args.driver, args.browser,
                          args.software_webgpu) as browser:
        environment = {
            "inputs": inputs, "browser": browser.capabilities,
            "software_webgpu": args.software_webgpu,
            "harness": {name: hashlib.sha256(Path(__file__).with_name(name).read_bytes()).hexdigest()
                        for name in ("browser_workbench.py", "check_browser_workbench.py")},
        }
        try:
            environment["webgpu"] = browser.webgpu_adapter()
            result = run(browser)
            browser.assert_no_errors()
            (browser.output / "result.json").write_text(json.dumps(result, indent=2), encoding="utf-8")
        except BaseException as error:
            failure = {"error": str(error), "diagnostic_errors": []}
            for collect in (lambda: browser.capture("failure-ui"), browser.assert_no_errors):
                try:
                    collect()
                except Exception as diagnostic_error:
                    failure["diagnostic_errors"].append(str(diagnostic_error))
            (browser.output / "failure.json").write_text(json.dumps(failure, indent=2), encoding="utf-8")
            raise
        finally:
            (browser.output / "environment.json").write_text(json.dumps(environment, indent=2), encoding="utf-8")
    print(json.dumps(result))


if __name__ == "__main__":
    main()
