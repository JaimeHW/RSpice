#!/usr/bin/env python3
"""Qualify the real browser workbench and its durable recovery workflow.

Serve the normal web entry and production worker, with the UI built using
`browser-qualification`. The optional observer exposes rendered controls only.
All edits use WebDriver input. Retain this run's output directory on failure.
"""

from __future__ import annotations

import argparse
from contextlib import contextmanager
import copy
import hashlib
import json
import math
from pathlib import Path
import time
import uuid

from browser_workbench import WorkbenchBrowser, controls, wait_for

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


def verify_clock_rejection(snapshot, operation, draft=None):
    rendered = controls(snapshot)
    expected = f"{operation} could not be timestamped".lower()
    if not any(expected in control["value"].lower() for control in rendered):
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


def reject_checkpoint_clock(browser, name):
    before = checkpoint_records(browser)
    for fault in CLOCK_FAULTS:
        with wall_clock(browser, fault):
            browser.click("Checkpoint now…", "button")
            verify_clock_rejection(browser.capture(f"{name}-clock-{fault}"), "Project checkpoint")
            if checkpoint_records(browser) != before:
                raise AssertionError("A failed timestamp operation changed durable checkpoint records")


def verify_age(browser, name, now_ms):
    for value, expected, suffix in ((now_ms - 86_400_000, "clock skew", "rollback"),
                                    (now_ms + 86_400_000, "1 d ago", "forward"),
                                    ("nonfinite", "time unavailable", "unavailable")):
        with wall_clock(browser, value):
            snapshot = browser.capture(f"{name}-age-{suffix}")
            if not any(control["value"] == expected or (
                    name == "checkpoint" and control["role"] == "button" and
                    control["label"].startswith(f"Manual checkpoint, {expected}, revision 1,"))
                    for control in controls(snapshot)):
                raise AssertionError(f"{name} did not report {expected!r} after a clock adjustment")


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
    choose_command(browser, "Check and save", keyboard=((note, ()),))
    snapshot = browser.capture("validated-save-preconditions")
    if not any(control["label"] == "Checks" and control["value"] == "0 blockers · 0 advisories"
               for control in controls(snapshot)):
        raise AssertionError("The authored circuit did not pass schematic validation")
    before = canonical_projects(browser)
    if before:
        raise AssertionError("A first-save fixture already contains a canonical project")
    for fault in CLOCK_FAULTS:
        with wall_clock(browser, fault):
            browser.click("Save validated revision", "button")
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
        browser.click("Save validated revision", "button")
        wait_for(lambda: any(control["value"] == "Validated revision saved"
                            for control in controls(browser.snapshot())), "the canonical validated save")
    finally:
        browser.script("window.showSaveFilePicker = window.__rspiceOriginalSavePicker; delete window.__rspiceOriginalSavePicker;")
        browser.record_input("file_picker_restored")
    finished_ms = time.time_ns() // 1_000_000
    saved_controls = controls(browser.capture("validated-save-complete"))
    for label, value in (("Saved snapshot", "3 / 3"), ("Saved source validated", "pass"),
                         ("Validated revision recorded", "pass")):
        if not any(control["label"] == label and control["value"] == value for control in saved_controls):
            raise AssertionError("The completed save did not display its validated snapshot status")
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
    browser.click("Close", "button")
    choose_command(browser, "Revision history")
    verify_age(browser, "revision", finished_ms)
    browser.click("Close", "button")
    browser.call("POST", "/refresh", {})
    wait_until_ready(browser)
    browser.click("Schematic canvas", "canvas")
    choose_command(browser, "Revision history")
    reloaded = browser.capture("validated-history-reloaded")
    if not any(control["value"] == note for control in controls(reloaded)):
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
    rendered = controls(browser.snapshot())
    if not any(control["value"] == "Engine messages will appear here." for control in rendered):
        browser.click("Clear console output", "button")
    before_save = canonical_projects(browser)
    started_ms = time.time_ns() // 1_000_000
    browser.click("Validate bindings", "button")
    snapshot = browser.capture("model-bindings-validated")
    if not any("Published durable model-validation receipt" in control["value"]
               for control in controls(snapshot)):
        raise AssertionError("Model validation did not publish a receipt")
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
    wait_for(lambda: any(control["value"] == "Saved" for control in controls(browser.snapshot())),
             "the accepted model-validation save")
    if (not started_ms <= receipt["validated_at_unix_ms"] <= finished_ms
            or receipt["platform"] != "browser-wasm32"):
        raise AssertionError("Model validation retained an incorrect timestamp or runtime")
    for fault in CLOCK_FAULTS:
        browser.click("Clear console output", "button")
        with wall_clock(browser, fault):
            browser.click("Validate bindings", "button")
            snapshot = browser.capture(f"model-validation-clock-{fault}")
            if not any("system clock cannot timestamp model validation" in control["value"]
                       for control in controls(snapshot)):
                raise AssertionError("Model validation did not report its clock failure")
        if any(control["value"] == "Unsaved changes" for control in controls(browser.snapshot())):
            raise AssertionError("A failed model validation dirtied the saved project")
        if canonical_projects(browser) != files:
            raise AssertionError("A failed validation replaced the canonical model receipt")

    # Also snapshot the working project: unchanged saved bytes alone could hide
    # an invalid receipt left in memory by the failed operation.
    choose_command(browser, "Open recovery center")
    before = checkpoint_records(browser)
    browser.click("Checkpoint now…", "button")
    records = wait_for(lambda: (current if (current := checkpoint_records(browser)) != before else None),
                       "the checkpoint after failed model validation")
    added = {key: value for key, value in records.items() if key not in before}
    snapshots = [json.loads(bytes(value)) for key, value in added.items() if key.endswith(".snapshot")]
    if len(snapshots) != 1 or snapshots[0]["execution_context"].get("model_validation_receipt") != receipt:
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
    wait_for(lambda: any(control["value"] == "Saved" for control in controls(browser.snapshot())),
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
            wait_for(lambda: any("Imported" in c["value"] and name in c["value"]
                                 for c in controls(browser.snapshot())), "the imported " + name + " source")
            browser.capture("provider-imported-" + name)
    finally:
        browser.cdp("Page.setInterceptFileChooserDialog", {"enabled": False})
    choose_command(browser, "Save all")
    wait_for(lambda: any(c["value"] == "Saved" for c in controls(browser.snapshot())),
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
                if not any("system clock cannot timestamp provider decision" in c["value"] for c in rendered):
                    raise AssertionError("Provider publication did not report its clock failure")
                if not any(c["role"] == "multilineTextInput" and c["value"] == draft for c in rendered):
                    raise AssertionError("Provider publication lost its editable audit reason")
                if not any(c["role"] == "radioButton" and c["label"] == provider
                           and c["toggled"] == "true" for c in rendered):
                    raise AssertionError("Provider publication lost its selected source")
            if canonical_projects(browser) != files:
                raise AssertionError("Failed provider publication changed the saved project")
            if any(c["value"] == "Unsaved changes" for c in controls(browser.snapshot())):
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
        wait_for(lambda: any(c["value"] == "Saved" for c in controls(browser.snapshot())),
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
    wait_for(lambda: any("Run 1" in c["value"] and "completed" in c["value"]
                        for c in controls(browser.snapshot())), "the saved-provider simulation", 120)
    browser.capture("provider-run-completed")
    choose_command(browser, "Open recovery center")
    before = checkpoint_records(browser)
    started_ms = time.time_ns() // 1_000_000
    browser.click("Checkpoint now…", "button")
    records = wait_for(lambda: (current if (current := checkpoint_records(browser)) != before else None),
                       "the saved-provider result checkpoint")
    added = {key: value for key, value in records.items() if key not in before}
    _, raw, project = verify_checkpoint(added, started_ms, time.time_ns() // 1_000_000)
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
    for waveform in waveforms:
        x, y = waveform["x"], waveform["y"]
        if (len(x) < 2 or len(x) != len(y) or not all(math.isfinite(v) for v in x + y)
                or any(b < a for a, b in zip(x, x[1:]))
                or not math.isclose(x[-1], 0.001, rel_tol=0, abs_tol=1e-12)):
            raise AssertionError("The saved-provider transient samples are incomplete or invalid")
    if not any(w["name"].upper().startswith("V(") and
               all(math.isclose(v, 5.0, rel_tol=0, abs_tol=1e-6) for v in w["y"])
               for w in waveforms):
        raise AssertionError("The saved-provider simulation did not retain the authored 5 V output")
    (browser.output / "model-provider-results.rspiceproj").write_bytes(raw)
    return {"worker_run": "passed", "waveforms": {w["name"]: len(w["x"]) for w in waveforms}}


def create_and_resolve_review(browser):
    choose_command(browser, "Review comments", input_burst=True)
    # The click and immediate editing input share one WebDriver request.
    browser.click("New comment", "button", keyboard=(("a", ("\ue009",)), (REVIEW_TITLE, ())))
    browser.click("Arm text tool", "button")
    browser.click("Schematic canvas", "canvas")
    browser.keys("\ue00c")
    choose_command(browser, "Review comments", keyboard=((REVIEW_BODY, ()),))
    reject_review_clock(browser, "Publish review update", REVIEW_BODY, "review-post")
    posted_start = time.time_ns() // 1_000_000
    browser.click("Publish review update", "button")
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


def verify_checkpoint(records, started_ms, finished_ms):
    manifests = [(key, json.loads(value)) for key, value in records.items()
                 if key.endswith(".manifest")]
    if len(manifests) != 1 or len(records) != 2:
        raise AssertionError(f"Expected one fresh checkpoint pair; got {list(records)}")
    key, manifest = manifests[0]
    if manifest["schema_version"] != 1 or manifest["reason"] != "manual":
        raise AssertionError(f"Unexpected checkpoint identity: {manifest}")
    if not started_ms <= manifest["created_unix_ms"] <= finished_ms:
        raise AssertionError(f"Checkpoint wall time is outside the observed creation interval: {manifest}")
    raw = bytes(records[key.removesuffix(".manifest") + ".snapshot"])
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
    query, name = "Browser configuration filter", "Browser configuration draft"
    choose_command(browser, "Configuration sets", keyboard=((query, ()),))

    def expect_value(snapshot, value, label=None):
        fields = [control for control in controls(snapshot) if control["role"] == "textInput"
                  and control["value"] == value and (label is None or control["label"] == label)]
        if len(fields) != 1:
            raise AssertionError(f"Configuration opening input was lost or misdirected: {value!r}")

    expect_value(browser.capture("configuration-filter"), query)
    browser.click("New configuration", "button", keyboard=(("a", ("\ue009",)), (name, ())))
    expect_value(browser.capture("configuration-name"), name, "Name")
    returned_query = "Configuration filter after Escape"
    browser.key_sequence((("\ue00c", ()), ("a", ("\ue009",)), (returned_query, ())))
    expect_value(browser.capture("configuration-cancelled"), returned_query)
    browser.click("Close", "button")


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


def run(browser):
    wait_until_ready(browser)
    browser.capture("startup")
    browser.click("Schematic canvas", "canvas")
    configuration_opening_input(browser)
    review_intervals = create_and_resolve_review(browser)
    choose_command(browser, "Open recovery center")
    wait_for(lambda: any(control["label"] == "Checkpoint now…"
                        for control in controls(browser.snapshot())), "the recovery workspace")
    if checkpoint_records(browser):
        raise AssertionError("A fresh profile already contains recovery records")
    reject_checkpoint_clock(browser, "empty-checkpoint-store")
    started_ms = time.time_ns() // 1_000_000
    browser.click("Checkpoint now…", "button")

    def published():
        records = checkpoint_records(browser)
        return records if any(key.endswith(".manifest") for key in records) else None

    records = wait_for(published, "durable checkpoint publication")
    (browser.output / "checkpoint-records.json").write_text(json.dumps(records, indent=2), encoding="utf-8")
    manifest, raw, project = verify_checkpoint(records, started_ms, time.time_ns() // 1_000_000)
    verify_review(project, review_intervals)
    browser.capture("checkpoint-created")
    reject_checkpoint_clock(browser, "retained-checkpoint-store")
    verify_age(browser, "checkpoint", time.time_ns() // 1_000_000)
    browser.click("Restore…", "button")

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
    choose_command(browser, "Revision history")
    wait_for(lambda: any("Project revision and audit history" in (control["label"], control["value"])
                        for control in controls(browser.snapshot())), "revision history")
    browser.capture("revision-history")
    validated_save = validated_save_clock(browser, review_intervals)
    model_validation = model_validation_clock(browser)
    provider_decision = provider_decision_clock(browser)
    return {"startup": "passed", "checkpoint_export": "passed", "revision_history": "opened",
            "configuration_opening_input": "passed",
            "review_and_checkpoint_clock_failures": {fault: "passed" for fault in CLOCK_FAULTS},
            "validated_save": validated_save,
            "model_validation": model_validation,
            "provider_decision": provider_decision,
            "review_post_and_resolution": "passed", "review_intervals_ms": review_intervals,
            "checkpoint": manifest, "export": exported.name, "project_version": project["version"]}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--web-root", type=Path, default=Path("crates/rspice-ui/web"))
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--driver")
    parser.add_argument("--browser")
    parser.add_argument("--software-webgpu", action="store_true",
                        help="Use SwiftShader for functional CI, without qualifying physical GPU support")
    args = parser.parse_args()
    inputs = {}
    for name in ("index.html", "simulation-worker.js", "automation-worker.js", "wasm-loader.js",
                 "pkg/rspice-ui.js", "pkg/rspice-ui_bg.wasm",
                 "pkg/rspice-ui-worker.js", "pkg/rspice-ui-worker_bg.wasm"):
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
