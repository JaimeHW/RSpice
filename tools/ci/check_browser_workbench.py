#!/usr/bin/env python3
"""Qualify the real browser workbench and its durable recovery workflow.

Serve the normal web entry and production worker, with the UI built using
`browser-qualification`. The optional observer exposes rendered controls only.
All edits use WebDriver input. Retain this run's output directory on failure.
"""

from __future__ import annotations

import argparse
import copy
import hashlib
import json
from pathlib import Path
import time
import uuid

from browser_workbench import WorkbenchBrowser, controls, wait_for

REVIEW_TITLE = "Browser recovery qualification"
REVIEW_BODY = "Verify that this review survives recovery export."
RESOLUTION = "Resolution verified in the browser."


def create_and_resolve_review(browser):
    choose_command(browser, "Review comments", input_burst=True)
    # The click and immediate editing input share one WebDriver request.
    browser.click("New comment", "button", keyboard=(("a", ("\ue009",)), (REVIEW_TITLE, ())))
    browser.click("Arm text tool", "button")
    browser.click("Schematic canvas", "canvas")
    browser.keys("\ue00c")
    choose_command(browser, "Review comments", keyboard=((REVIEW_BODY, ()),))
    posted_start = time.time_ns() // 1_000_000
    browser.click("Publish review update", "button")
    browser.capture("review-posted")
    posted_end = time.time_ns() // 1_000_000
    browser.click("", "multilineTextInput")
    browser.keys(RESOLUTION)
    resolved_start = time.time_ns() // 1_000_000
    browser.click("Resolve with note", "button")
    browser.capture("review-resolved")
    resolved_end = time.time_ns() // 1_000_000
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


def run(browser):
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
    browser.capture("startup")
    browser.click("Schematic canvas", "canvas")
    configuration_opening_input(browser)
    review_intervals = create_and_resolve_review(browser)
    choose_command(browser, "Open recovery center")
    wait_for(lambda: any(control["label"] == "Checkpoint now…"
                        for control in controls(browser.snapshot())), "the recovery workspace")
    if checkpoint_records(browser):
        raise AssertionError("A fresh profile already contains recovery records")
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
    return {"startup": "passed", "checkpoint_export": "passed", "revision_history": "opened",
            "configuration_opening_input": "passed",
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
    for name in ("index.html", "simulation-worker.js", "automation-worker.js",
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
            for collect in (lambda: browser.capture("failure"), browser.assert_no_errors):
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
