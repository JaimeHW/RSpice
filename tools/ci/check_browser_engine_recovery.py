#!/usr/bin/env python3
"""Qualify browser worker startup failure, queued-run termination and UI retry.

The first real worker WASM response is held until a deck is queued, then fails
with HTTP 503. Later requests receive the unmodified production worker assets.
All application actions use real keyboard/pointer input and read-only observation.
"""

from __future__ import annotations

import argparse
import hashlib
import http.server
import json
import math
from pathlib import Path
import threading
import time
from urllib.parse import urlsplit

from browser_workbench import WorkbenchBrowser, controls, wait_for
from check_browser_workbench import choose_command, checkpoint_records, verify_checkpoint

DECK = "* Engine recovery analytic check\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n"


class WorkerLoadFault:
    def __init__(self):
        self.requested = threading.Event()
        self.release = threading.Event()
        self.lock = threading.Lock()
        self.requests = []

    def handler(self):
        fault = self

        class Handler(http.server.SimpleHTTPRequestHandler):
            def do_GET(self):
                if urlsplit(self.path).path == "/pkg/rspice-ui-worker_bg.wasm":
                    with fault.lock:
                        first = not fault.requests
                        record = {"path": self.path, "status": None}
                        fault.requests.append(record)
                    if first:
                        fault.requested.set()
                        released = fault.release.wait(120)
                        record["released"] = released
                        record["status"] = 503
                        try:
                            self.send_error(503, "Controlled worker initialization failure")
                        except (BrokenPipeError, ConnectionResetError, ConnectionAbortedError) as error:
                            # Terminating a loading worker can close its HTTP
                            # connection before this held response is written.
                            record["connection_closed"] = str(error)
                        return
                    record["status"] = 200
                super().do_GET()

        return Handler


def has_control(browser, label):
    return any(control["label"] == label for control in controls(browser.snapshot()))


def author_deck(browser):
    browser.click("Netlist & Script Editor", "button")
    browser.click("New source document", "button")
    browser.click("Logical path", "textInput")
    # The transaction's Create action is inside a scrollable manager body.
    # Keyboard traversal scrolls it into view on the next rendered frame.
    browser.keys("\ue004")
    browser.snapshot()
    browser.keys("\ue007")
    browser.snapshot()
    browser.click("Close", "button")
    browser.click("Project SPICE netlist editor", "multilineTextInput")
    browser.keys("a", ("\ue009",))
    browser.snapshot()
    browser.keys(DECK)
    browser.click("Validate source", "button")
    browser.capture("validated-deck")


def check_console(browser, fault):
    entries = browser.console_entries()
    errors = browser.script("return window.__rspiceQualificationErrors || []")
    for entry in entries:
        if entry["level"] != "SEVERE":
            continue
        expected = any(record["status"] == 503
                       and browser.origin + record["path"] in entry["message"]
                       and "503" in entry["message"] for record in fault.requests)
        if not expected:
            errors.append(entry["message"])
    if errors:
        raise AssertionError("Unexpected browser errors: " + "\n".join(errors))


def run(browser, fault, cancel_startup=False):
    wait_for(lambda: browser.script("return !document.getElementById('rspice_loading')"),
             "the real workbench", 120)
    if not fault.requested.is_set():
        raise AssertionError("The real worker did not request its WASM module")
    if not has_control(browser, "Engine starting"):
        raise AssertionError("An uninitialized worker was not reported as Engine starting")
    browser.capture("starting")
    author_deck(browser)
    browser.click("Run deck", "button")
    wait_for(lambda: has_control(browser, "Stop"), "the queued manual deck")
    if not has_control(browser, "Engine starting"):
        raise AssertionError("A queued request advertised progress before worker initialization")
    browser.capture("queued-before-failure")
    if cancel_startup:
        browser.click("Stop", "button")
    fault.release.set()
    retry_label = "Engine stopped" if cancel_startup else "Engine unavailable"
    wait_for(lambda: has_control(browser, retry_label), "retirement of the initial worker")
    wait_for(lambda: has_control(browser, "Run deck"), "termination of the queued run")
    browser.capture("retired")
    browser.click(retry_label, "button")
    wait_for(lambda: has_control(browser, "Engine ready"), "explicit engine retry", 120)
    if has_control(browser, "Stop"):
        raise AssertionError("Retrying engine startup queued a simulation")
    if not browser.script("return window.__RSPICE_SIM_WORKER_READY === true"):
        raise AssertionError("Retry did not publish actual worker readiness")
    if browser.script("return document.documentElement.getAttribute('data-rspice-wasm-jit-status')") != "qualified":
        raise AssertionError("The replacement worker did not publish its JIT qualification")
    browser.capture("recovered")
    # Prepared execution authorization is consumed by the first launch.
    browser.click("Validate source", "button")
    browser.click("Run deck", "button")
    wait_for(lambda: any("Run 2" in control["value"] and "completed" in control["value"]
                        for control in controls(browser.snapshot())), "the recovered run's results")
    browser.capture("completed")
    choose_command(browser, "Open recovery center")
    wait_for(lambda: has_control(browser, "Checkpoint now…"), "the recovery workspace")
    started_ms = time.time_ns() // 1_000_000
    browser.click("Checkpoint now…", "button")

    def published():
        records = checkpoint_records(browser)
        return records if any(key.endswith(".manifest") for key in records) else None

    records = wait_for(published, "a persisted result checkpoint")
    (browser.output / "checkpoint-records.json").write_text(json.dumps(records, indent=2), encoding="utf-8")
    _, _, project = verify_checkpoint(records, started_ms, time.time_ns() // 1_000_000)
    runs = sorted(project["simulation_results"]["runs"], key=lambda run: run["id"])
    if ([run["id"] for run in runs] != [1, 2]
            or [run["success"] for run in runs] != [False, True]):
        raise AssertionError("Expected one unsuccessful run followed by one successful retry")
    failed, completed = runs
    initial_lifecycle = "aborted" if cancel_startup else "failed"
    if failed["lifecycle"] != initial_lifecycle or completed["lifecycle"] != "completed":
        raise AssertionError("Worker failure/retry did not preserve terminal run lifecycles")
    if not cancel_startup and (len(failed["analyses"]) != 1 or not failed["analyses"][0].get("error_message")):
        raise AssertionError("The failed request did not retain one diagnostic analysis")
    if len(completed["analyses"]) != 1:
        raise AssertionError("Retry duplicated or omitted an analysis")
    operating_point = completed["analyses"][0]["dc_op"]
    voltages = {value["name"]: value["value"] for value in operating_point["node_voltages"]}
    currents = {value["name"]: value["value"] for value in operating_point["branch_currents"]}
    if not math.isclose(voltages["V(IN)"], 1.0, rel_tol=0, abs_tol=1e-9):
        raise AssertionError(f"Recovered solver produced the wrong operating point: {voltages}")
    if not math.isclose(currents["I(V1)"], -0.001, rel_tol=0, abs_tol=1e-12):
        raise AssertionError(f"Recovered solver produced the wrong source current: {currents}")
    if len(fault.requests) != 2 or not fault.requests[0]["released"]:
        raise AssertionError(f"Unexpected worker request sequence: {fault.requests}")
    return {"initial_lifecycle": initial_lifecycle, "explicit_retry": "passed",
            "recovered_run": "passed", "node_voltages": voltages,
            "branch_currents": currents, "requests": fault.requests}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--web-root", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--driver")
    parser.add_argument("--browser")
    parser.add_argument("--software-webgpu", action="store_true")
    parser.add_argument("--cancel-startup", action="store_true",
                        help="Stop the queued run before releasing its delayed worker response")
    args = parser.parse_args()
    inputs = {str(path.relative_to(args.web_root)): hashlib.sha256(path.read_bytes()).hexdigest()
              for path in args.web_root.rglob("*") if path.is_file()}
    fault = WorkerLoadFault()
    with WorkbenchBrowser(args.web_root, args.output, args.driver, args.browser,
                          args.software_webgpu, fault.handler()) as browser:
        adapter = None
        try:
            adapter = browser.webgpu_adapter()
            result = run(browser, fault, args.cancel_startup)
            check_console(browser, fault)
            (browser.output / "result.json").write_text(json.dumps(result, indent=2), encoding="utf-8")
        except BaseException as error:
            failure = {"error": str(error), "diagnostic_errors": []}
            fault.release.set()
            for collect in (lambda: browser.capture("failure"), lambda: check_console(browser, fault)):
                try:
                    collect()
                except Exception as diagnostic_error:
                    failure["diagnostic_errors"].append(str(diagnostic_error))
            (browser.output / "failure.json").write_text(json.dumps(failure, indent=2), encoding="utf-8")
            raise
        finally:
            fault.release.set()
            environment = {"inputs": inputs, "browser": browser.capabilities,
                           "webgpu": adapter,
                           "software_webgpu": args.software_webgpu, "requests": fault.requests,
                           "harness": {name: hashlib.sha256(Path(__file__).with_name(name).read_bytes()).hexdigest()
                                       for name in ("browser_workbench.py", "check_browser_workbench.py",
                                                    "check_browser_engine_recovery.py")}}
            (browser.output / "environment.json").write_text(json.dumps(environment, indent=2), encoding="utf-8")
    print(json.dumps(result))


if __name__ == "__main__":
    main()
