"""Real Chrome/WebDriver input and read-only egui observation for UI qualification.

Each instance owns an isolated HTTP origin, browser profile and download directory.
No application state or authorization is injected. Only the optional rendered-control
observer is queried; pointer and keyboard actions travel through the browser backend.
"""

from __future__ import annotations

import base64
import functools
import http.server
import json
import os
from pathlib import Path
import shutil
import socket
import subprocess
import tempfile
import threading
import time
import urllib.error
import urllib.request

from check_wasm_jit_browser import find_chromium


def wait_for(read, description: str, timeout: float = 30):
    deadline = time.monotonic() + timeout
    while True:
        value = read()
        if value:
            return value
        if time.monotonic() >= deadline:
            raise AssertionError(f"Timed out waiting for {description}")
        time.sleep(0.1)


def multiply(a, b):
    """Compose the AccessKit affine transforms in parent-to-child order."""
    return (
        a[0] * b[0] + a[2] * b[1], a[1] * b[0] + a[3] * b[1],
        a[0] * b[2] + a[2] * b[3], a[1] * b[2] + a[3] * b[3],
        a[0] * b[4] + a[2] * b[5] + a[4],
        a[1] * b[4] + a[3] * b[5] + a[5],
    )


def controls(snapshot):
    update = snapshot["tree"]
    nodes = dict(update["nodes"])
    result = []
    visited = set()

    def visit(identity, parent):
        if identity in visited:
            raise AssertionError("Rendered control tree repeats a node")
        visited.add(identity)
        node = nodes[identity]
        properties = node.get("properties", {})
        transform = multiply(parent, properties.get("transform", (1, 0, 0, 1, 0, 0)))
        bounds = properties.get("bounds")
        if bounds:
            x = (bounds["x0"] + bounds["x1"]) / 2
            y = (bounds["y0"] + bounds["y1"]) / 2
            result.append({
                "id": str(identity), "role": node["role"],
                "label": properties.get("label", ""),
                "value": properties.get("value", ""),
                "actions": node.get("actions", 0),
                "center": (transform[0] * x + transform[2] * y + transform[4],
                           transform[1] * x + transform[3] * y + transform[5]),
            })
        for child in properties.get("children", []):
            visit(child, transform)

    visit(update["tree"]["root"], (1, 0, 0, 1, 0, 0))
    return result


def key_actions(sequence):
    """Build one ordered WebDriver sequence, including modifier releases."""
    actions = []
    for text, modifiers in sequence:
        actions.extend({"type": "keyDown", "value": modifier} for modifier in modifiers)
        for character in text:
            actions.extend([{"type": "keyDown", "value": character},
                            {"type": "keyUp", "value": character}])
        actions.extend({"type": "keyUp", "value": modifier} for modifier in reversed(modifiers))
    return actions


class WorkbenchBrowser:
    def __init__(self, web_root: Path, output: Path, driver: str | None = None,
                 browser: str | None = None, software_webgpu: bool = False):
        self.web_root = web_root.resolve()
        self.output = output.resolve()
        self.output.mkdir(parents=True, exist_ok=True)
        if any(self.output.iterdir()):
            raise FileExistsError(f"Qualification output must be empty: {self.output}")
        self.driver = driver or shutil.which("chromedriver")
        if not self.driver:
            raise RuntimeError("A matching ChromeDriver is required; pass --driver")
        self.browser = browser or find_chromium()
        self.software_webgpu = software_webgpu
        self.session = None
        self.process = None
        self.server = None
        self.thread = None
        self.profile = None
        self.driver_log = None
        self.sequence = 0

    def __enter__(self):
        try:
            self._start()
            return self
        except BaseException as error:
            self.__exit__(type(error), error, error.__traceback__)
            raise

    def _start(self):
        handler = functools.partial(http.server.SimpleHTTPRequestHandler,
                                    directory=str(self.web_root))
        self.server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), handler)
        self.thread = threading.Thread(target=self.server.serve_forever, daemon=True)
        self.thread.start()
        self.origin = f"http://127.0.0.1:{self.server.server_port}"
        self.profile = tempfile.TemporaryDirectory(prefix="rspice-workbench-")
        self.downloads = self.output / "downloads"
        self.downloads.mkdir(exist_ok=True)
        with socket.socket() as reserved:
            reserved.bind(("127.0.0.1", 0))
            port = reserved.getsockname()[1]
        self.endpoint = f"http://127.0.0.1:{port}"
        self.driver_log = (self.output / "webdriver.log").open("w", encoding="utf-8")
        self.process = subprocess.Popen(
            [self.driver, f"--port={port}", "--allowed-ips=127.0.0.1"],
            stdout=self.driver_log, stderr=subprocess.STDOUT,
            creationflags=subprocess.CREATE_NO_WINDOW if os.name == "nt" else 0,
        )

        def ready():
            if self.process.poll() is not None:
                raise RuntimeError("ChromeDriver exited before startup; see webdriver.log")
            try:
                return self.command("GET", "/status").get("ready")
            except urllib.error.URLError:
                return False

        wait_for(ready, "ChromeDriver startup")
        arguments = ["--headless=new", "--no-first-run", "--no-default-browser-check",
                     "--disable-background-networking", "--disable-default-apps",
                     "--disable-extensions", "--disable-sync", "--mute-audio",
                     "--window-size=1440,1000",
                     f"--user-data-dir={self.profile.name}"]
        if os.name != "nt" and getattr(os, "geteuid", lambda: -1)() == 0:
            arguments.append("--no-sandbox")
        if self.software_webgpu:
            # Select Dawn's software adapter without changing the compositor.
            arguments.extend(["--enable-unsafe-webgpu", "--use-webgpu-adapter=swiftshader"])
        response = self.command("POST", "/session", {"capabilities": {"alwaysMatch": {
            "browserName": "chrome", "goog:loggingPrefs": {"browser": "ALL"},
            "goog:chromeOptions": {"binary": self.browser, "args": arguments,
                "prefs": {"download.default_directory": str(self.downloads),
                          "download.prompt_for_download": False}},
        }}})
        self.session = response["sessionId"]
        self.capabilities = response["capabilities"]
        self.cdp("Browser.setDownloadBehavior", {"behavior": "allow", "downloadPath": str(self.downloads)})
        self.cdp("Page.addScriptToEvaluateOnNewDocument", {"source": """
            window.__rspiceQualificationErrors = [];
            addEventListener('error', event => {
                window.__rspiceQualificationErrors.push(String(event.message || 'Resource load failed'));
            });
            addEventListener('unhandledrejection', event => {
                window.__rspiceQualificationErrors.push(String(event.reason));
            });
        """})
        self.navigate(self.origin)

    def command(self, method, path, payload=None):
        data = None if payload is None else json.dumps(payload).encode()
        request = urllib.request.Request(self.endpoint + path, data=data, method=method,
                                         headers={"Content-Type": "application/json"})
        try:
            with urllib.request.urlopen(request, timeout=180) as response:
                decoded = json.load(response)
        except urllib.error.HTTPError as error:
            raise RuntimeError(f"WebDriver {method} {path}: {error.read().decode()}") from error
        return decoded["value"]

    def call(self, method, path, payload=None):
        return self.command(method, f"/session/{self.session}{path}", payload)

    def cdp(self, command, params):
        return self.call("POST", "/goog/cdp/execute", {"cmd": command, "params": params})

    def script(self, script, *args):
        return self.call("POST", "/execute/sync", {"script": script, "args": list(args)})

    def record_input(self, action, **details):
        record = {"action": action, "unix_ms": time.time_ns() // 1_000_000,
                  "monotonic_ns": time.monotonic_ns(), **details}
        with (self.output / "input.jsonl").open("a", encoding="utf-8") as stream:
            stream.write(json.dumps(record) + "\n")

    def navigate(self, url):
        self.record_input("navigate", url=url)
        self.call("POST", "/url", {"url": url})

    def snapshot(self):
        self.sequence += 1
        request = str(self.sequence)
        self.script("""
            document.documentElement.setAttribute('data-rspice-qualification-request', arguments[0]);
        """, request)

        def current():
            errors = self.script("return window.__rspiceQualificationErrors || []")
            if errors:
                raise AssertionError("Browser failed before observation: " + "\n".join(errors))
            raw = self.script("return document.getElementById('rspice_qualification_snapshot')?.textContent;")
            if not raw:
                return None
            snapshot = json.loads(raw)
            return snapshot if snapshot["request"] == request else None

        return wait_for(current, "a fresh rendered-control snapshot", 90)

    def click(self, label, role=None, *, keyboard=()):
        candidates = [control for control in controls(self.snapshot())
                      if control["label"] == label and (role is None or control["role"] == role)]
        if len(candidates) != 1:
            raise AssertionError(f"Expected one rendered {role or 'control'} named {label!r}; got {candidates}")
        point = candidates[0]["center"]
        canvas = self.script("""
            const r = document.getElementById('rspice_canvas').getBoundingClientRect();
            return {x: r.x, y: r.y, ratio: devicePixelRatio};
        """)
        x, y = (round(canvas[axis] + point[i] / canvas["ratio"])
                for i, axis in enumerate(("x", "y")))
        self.record_input("click", label=label, role=role, node=candidates[0]["id"], x=x, y=y,
                          keyboard=keyboard)
        pointer = [
                {"type": "pointerMove", "duration": 0, "origin": "viewport", "x": x, "y": y},
                {"type": "pointerDown", "button": 0}, {"type": "pause", "duration": 60},
                {"type": "pointerUp", "button": 0},
        ]
        sources = [{"type": "pointer", "id": "mouse",
                    "parameters": {"pointerType": "mouse"}, "actions": pointer}]
        if keyboard:
            # W3C sources advance together by tick: begin typing directly after
            # release, without a rendered-control wait between click and text.
            sources.append({"type": "key", "id": "keyboard", "actions":
                            [{"type": "pause", "duration": 0} for _ in pointer] + key_actions(keyboard)})
        self.call("POST", "/actions", {"actions": sources})
        self.snapshot()

    def keys(self, text, modifiers=()):
        self.key_sequence(((text, modifiers),))

    def key_sequence(self, sequence):
        self.record_input("keys", sequence=sequence)
        self.call("POST", "/actions", {"actions": [{"type": "key", "id": "keyboard",
                                                   "actions": key_actions(sequence)}]})

    def capture(self, name):
        try:
            snapshot = self.snapshot()
        finally:
            # Startup errors can prevent an egui frame; retain the visible page.
            image = self.call("GET", "/screenshot")
            (self.output / f"{name}.png").write_bytes(base64.b64decode(image, validate=True))
        (self.output / f"{name}.json").write_text(json.dumps(snapshot, indent=2), encoding="utf-8")
        return snapshot

    def assert_no_errors(self):
        entries = self.call("POST", "/se/log", {"type": "browser"})
        (self.output / "console.json").write_text(json.dumps(entries, indent=2), encoding="utf-8")
        errors = self.script("return window.__rspiceQualificationErrors || []")
        errors.extend(entry["message"] for entry in entries if entry["level"] == "SEVERE")
        if errors:
            raise AssertionError("Browser errors: " + "\n".join(errors))

    def __exit__(self, exception_type, *_):
        failures = []

        def attempt(action):
            try:
                action()
            except Exception as error:
                failures.append(str(error))

        if self.session:
            attempt(lambda: self.call("DELETE", ""))

        def stop_driver():
            if self.process:
                self.process.terminate()
                try:
                    self.process.wait(timeout=10)
                except subprocess.TimeoutExpired:
                    self.process.kill()
                    self.process.wait(timeout=10)

        attempt(stop_driver)
        if self.driver_log:
            attempt(self.driver_log.close)
        if self.server:
            attempt(self.server.shutdown)
            attempt(self.server.server_close)
        if self.thread:
            attempt(lambda: self.thread.join(timeout=5))
        if self.profile:
            attempt(self.profile.cleanup)
        if failures:
            (self.output / "cleanup-errors.json").write_text(json.dumps(failures), encoding="utf-8")
            if exception_type is None:
                raise RuntimeError("Browser cleanup failed: " + "; ".join(failures))
