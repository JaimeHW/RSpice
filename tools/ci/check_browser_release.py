"""Qualify packaged UI startup, playground solves, and the pinned Python worker."""

import argparse
import base64
import hashlib
import http.server
import json
import os
import fnmatch
from pathlib import Path
import re
import subprocess
import urllib.parse

from browser_workbench import WorkbenchBrowser, wait_for


AUTOMATION_PAGE = "/__rspice_qualification__/automation.html"


class ReleaseHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        # Reproduce the packaged static-host headers, especially the CSP. A
        # permissive local server cannot qualify runtime startup under that policy.
        rules = (Path(self.directory) / "_headers").read_text(encoding="utf-8")
        for name, value in release_headers(rules, urllib.parse.urlsplit(self.path).path):
            self.send_header(name, value)
        super().end_headers()

    def do_GET(self):
        if urllib.parse.urlsplit(self.path).path != AUTOMATION_PAGE:
            return super().do_GET()
        payload = Path(__file__).with_name("automation_browser_debugger_harness.html").read_bytes()
        self.send_response(200)
        self.send_header("Content-Type", "text/html; charset=utf-8")
        self.send_header("Content-Length", str(len(payload)))
        self.end_headers()
        self.wfile.write(payload)


def release_headers(rules, path):
    matching = False
    result = []
    for line in rules.splitlines():
        if not line.strip() or line.lstrip().startswith("#"):
            continue
        if not line[0].isspace():
            matching = fnmatch.fnmatchcase(path, line.strip())
        elif matching:
            name, separator, value = line.strip().partition(":")
            if not separator or not name or not value.strip():
                raise ValueError("Invalid packaged response header: " + line)
            result.append((name, value.strip()))
    return result


def startup_ready(state):
    if state.get("error"):
        raise AssertionError(state["error"])
    return bool(state.get("canvas") and not state.get("loading") and state.get("worker_ready"))


def verify_worker_urls(state, asset_url):
    for key, name in (("simulation_worker", "simulation-worker.js"),
                      ("automation_worker", "automation-worker.js")):
        if state.get(key) != asset_url + name:
            raise AssertionError(f"{key} is not bound to the packaged asset cohort: {state.get(key)}")


def read_startup(browser):
    return browser.script("""
        const canvas = document.getElementById('rspice_canvas');
        const loading = document.getElementById('rspice_loading');
        return {
            canvas: !!canvas && canvas.width > 0 && canvas.height > 0,
            loading: !!loading,
            error: window.__RSPICE_SIM_WORKER_ERROR ||
                (loading?.getAttribute('role') === 'alert' ? loading.textContent : null),
            worker_ready: window.__RSPICE_SIM_WORKER_READY === true,
            simulation_worker: window.__RSPICE_SIM_WORKER_URL,
            automation_worker: window.__RSPICE_AUTOMATION_WORKER_URL,
            jit: document.documentElement.getAttribute('data-rspice-wasm-jit-status'),
            jit_capability: window.__RSPICE_WASM_JIT_CAPABILITY,
            width: innerWidth, height: innerHeight,
        };
    """)


def capture(browser, name, state):
    (browser.output / f"{name}.json").write_text(json.dumps(state, indent=2), encoding="utf-8")
    image = browser.call("GET", "/screenshot")
    (browser.output / f"{name}.png").write_bytes(base64.b64decode(image, validate=True))


def open_page(browser, url, width, height):
    # Startup cases are independent pages. Retain earlier tabs instead of
    # navigating away from an unsaved project or bypassing its unload guard.
    context = browser.call("POST", "/window/new", {"type": "tab"})
    browser.call("POST", "/window", {"handle": context["handle"]})
    browser.cdp("Emulation.setDeviceMetricsOverride", {
        "width": width, "height": height, "deviceScaleFactor": 1, "mobile": False})
    browser.navigate(url)


def playground_layout(browser):
    state = browser.script("""
        const width = document.documentElement.clientWidth;
        const ids = ['btn-summary', 'btn-op', 'btn-ac', 'btn-tran',
                     'tstop', 'hmax', 'fstart', 'fstop', 'fpoints'];
        return {
            width, scroll_width: document.documentElement.scrollWidth,
            controls: ids.map(id => {
                const element = document.getElementById(id);
                const bounds = element?.getBoundingClientRect();
                return {id, visible: !!element?.getClientRects().length,
                        left: bounds?.left, right: bounds?.right};
            }),
        };
    """)
    clipped = [control for control in state["controls"]
               if not control["visible"] or control["left"] < -1
               or control["right"] > state["width"] + 1]
    if state["scroll_width"] > state["width"] + 1 or clipped:
        raise AssertionError(f"Playground controls overflow the viewport: {state}")
    return state


def playground_result(browser):
    state = browser.script(r"""
        const notices = [...document.querySelectorAll('#log .ok')].map(node => node.textContent);
        const points = notices.map(text => text.match(/^tran:.*?· (\d+) pts/)).find(Boolean);
        return {
            points: points ? Number(points[1]) : 0,
            errors: [...document.querySelectorAll('#log .err')].map(node => node.textContent),
            traces: [...document.querySelectorAll('#plot path')].map(path => {
                const data = path.getAttribute('d') || '';
                return { points: (data.match(/[ML]/g) || []).length,
                         finite: !/NaN|Infinity|undefined/.test(data) };
            }),
        };
    """)
    if (state["points"] < 2 or state["errors"] or not state["traces"]
            or any(trace["points"] < 2 or not trace["finite"] for trace in state["traces"])):
        raise AssertionError(f"Playground did not render measured transient samples: {state}")
    return state


def release_inputs(root):
    inputs = {"_headers": hashlib.sha256((root / "_headers").read_bytes()).hexdigest()}
    for path in sorted([*(root / "ide").rglob("*"), *(root / "play").rglob("*")]):
        if path.is_symlink():
            raise ValueError(f"release input must not be a symlink: {path}")
        if path.is_file():
            inputs[path.relative_to(root).as_posix()] = hashlib.sha256(path.read_bytes()).hexdigest()
    return inputs


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--web-root", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--driver", default=os.environ.get("CHROMEDRIVER"))
    parser.add_argument("--browser", default=os.environ.get("CHROME"))
    parser.add_argument("--software-webgpu", action="store_true")
    args = parser.parse_args()
    build = json.loads((args.web_root / "build.json").read_text(encoding="utf-8"))
    identity = build.get("ide_executable_asset_sha256", "")
    if not re.fullmatch(r"[0-9a-f]{64}", identity):
        raise ValueError("release must name an immutable IDE asset cohort")
    inputs = release_inputs(args.web_root)
    report = {"schema_version": 2, "build": build, "inputs": inputs, "viewports": [],
              "qualification_source_sha": subprocess.check_output(
                  ["git", "-C", str(Path(__file__).resolve().parents[2]), "rev-parse", "HEAD"],
                  text=True).strip(),
              "scope": "UI startup at emulated viewports, playground solves, and Python debugger integration",
              "software_webgpu": args.software_webgpu,
              "harness": {name: hashlib.sha256(Path(__file__).with_name(name).read_bytes()).hexdigest()
                          for name in ("check_browser_release.py", "browser_workbench.py",
                                       "automation_browser_debugger_harness.html")}}
    with WorkbenchBrowser(args.web_root, args.output, args.driver, args.browser,
                          args.software_webgpu, request_handler=ReleaseHandler) as browser:
        try:
            report["browser"] = browser.capabilities
            asset_url = f"{browser.origin}/ide/assets/{identity}/"
            for label, width, height in (("desktop", 1280, 900), ("portrait", 820, 1180), ("compact", 390, 844)):
                open_page(browser, browser.origin + "/ide/", width, height)

                def ready():
                    state = read_startup(browser)
                    return state if startup_ready(state) else None

                state = wait_for(ready, f"{label} packaged UI startup", timeout=120)
                verify_worker_urls(state, asset_url)
                if state["jit"] != "qualified":
                    raise AssertionError(f"packaged worker did not qualify its JIT: {state}")
                browser.assert_no_errors()
                capture(browser, label, state)
                report["viewports"].append({"label": label, **state})
                open_page(browser, browser.origin + "/play/", width, height)
                wait_for(lambda: browser.script("""
                    return document.body.innerText.includes('worker ready') &&
                        document.body.innerText.includes('solved in');
                """), f"{label} playground transient solve", timeout=120)
                browser.assert_no_errors()
                capture(browser, label + "-playground", {
                    "body": browser.script("return document.body.innerText"),
                    "layout": playground_layout(browser),
                    "result": playground_result(browser),
                })
            report["webgpu"] = browser.webgpu_adapter()
            open_page(browser, browser.origin + AUTOMATION_PAGE + "?" + urllib.parse.urlencode(
                {"worker": asset_url + "automation-worker.js"}), 1280, 900)
            automation = wait_for(lambda: browser.script(
                "return window.RSPICE_AUTOMATION_DEBUGGER_QUALIFICATION || null"),
                "packaged Python debugger qualification", timeout=150)
            capture(browser, "automation", automation)
            if automation.get("status") != "complete" or automation.get("evaluated_value") != 42:
                raise AssertionError(f"packaged Python qualification failed: {automation}")
            browser.assert_no_errors()
            report["automation"] = automation
            if inputs != release_inputs(args.web_root):
                raise AssertionError("Packaged inputs changed during browser qualification")
            report["status"] = "passed"
        except BaseException as error:
            report["status"] = "failed"
            report["error"] = str(error)
            report["diagnostic_errors"] = []
            for collect in (lambda: capture(browser, "failure", read_startup(browser)), browser.assert_no_errors):
                try:
                    collect()
                except Exception as diagnostic_error:
                    report["diagnostic_errors"].append(str(diagnostic_error))
            raise
        finally:
            (browser.output / "release.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
    print(json.dumps({"status": report["status"], "evidence": str(browser.output)}))


if __name__ == "__main__":
    main()
