#!/usr/bin/env python3
"""Run the optimized browser WASM JIT solver gate in headless Chromium."""

from __future__ import annotations

import argparse
import functools
import http.server
import pathlib
import subprocess
import shutil
import tempfile
import threading
import urllib.parse

EXPECTED_STAMPS = 20000

# The page runs on a real clock, so the runner cannot bound it with
# --virtual-time-budget and read the DOM afterwards: a virtual clock reports a
# stamp cost of zero and races past the page's own timeout. It reports its
# verdict here instead, and this is how long the whole startup -- a 17 MB
# module fetched, compiled and instantiated, then two models compiled -- is
# allowed to take on a cold runner.
VERDICT_TIMEOUT_SECONDS = 180.0
VERDICT_PATH = "/rspice-qualification"

# Deliberately loose. The regression worth catching here is structural -- a
# capability bound to a wasm-bindgen wrapper rather than a raw export, or a
# model that stopped fusing -- and each costs a JavaScript call per operation
# or per contribution, which is orders of magnitude. A tight ceiling would
# instead track how loaded the CI runner happened to be.
MAX_NANOSECONDS_PER_STAMP = 25000.0

VERDICT: dict[str, str] = {}
VERDICT_ARRIVED = threading.Event()


class QualificationHandler(http.server.SimpleHTTPRequestHandler):
    def log_message(self, _format: str, *_args: object) -> None:
        pass

    def do_GET(self) -> None:  # noqa: N802 - http.server's spelling
        parsed = urllib.parse.urlparse(self.path)
        if parsed.path != VERDICT_PATH:
            super().do_GET()
            return
        VERDICT.update(
            {key: values[0] for key, values in urllib.parse.parse_qs(parsed.query).items()}
        )
        self.send_response(204)
        self.end_headers()
        VERDICT_ARRIVED.set()


def read_nanoseconds_per_stamp(reported: str | None) -> float:
    if reported is None:
        raise SystemExit("browser WASM JIT qualification reported no stamp timing")
    nanoseconds = float(reported)
    if nanoseconds <= 0.0:
        raise SystemExit(
            "browser WASM JIT qualification reported a zero stamp cost, which "
            "means the measurement did not run rather than that it was fast"
        )
    return nanoseconds


def find_chromium() -> str:
    for candidate in (
        "google-chrome",
        "google-chrome-stable",
        "chromium",
        "chromium-browser",
        "chrome",
    ):
        executable = shutil.which(candidate)
        if executable:
            return executable
    # CI runs this on Linux, but a gate is only useful if it can be reproduced
    # where the code is written, so the desktop install locations count too.
    for installed in (
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        "C:/Program Files/Google/Chrome/Application/chrome.exe",
        "C:/Program Files (x86)/Google/Chrome/Application/chrome.exe",
    ):
        path = pathlib.Path(installed)
        if path.is_file():
            return str(path)
    raise SystemExit("Chrome/Chromium is required for the browser WASM JIT qualification")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--web-root",
        type=pathlib.Path,
        default=pathlib.Path("crates/rspice-ui/web"),
    )
    args = parser.parse_args()
    web_root = args.web_root.resolve()
    required = (
        web_root / "wasm-jit-qualification.html",
        web_root / "simulation-worker.js",
        web_root / "pkg" / "rspice-ui-worker.js",
        web_root / "pkg" / "rspice-ui-worker_bg.wasm",
    )
    missing = [str(path) for path in required if not path.is_file()]
    if missing:
        raise SystemExit("browser WASM JIT qualification is missing: " + ", ".join(missing))

    handler = functools.partial(QualificationHandler, directory=str(web_root))
    server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), handler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    url = f"http://127.0.0.1:{server.server_port}/wasm-jit-qualification.html"
    with tempfile.TemporaryDirectory(prefix="rspice-wasm-jit-chrome-") as profile:
        browser = subprocess.Popen(
            [
                find_chromium(),
                "--headless=new",
                "--no-sandbox",
                "--disable-gpu",
                "--disable-dev-shm-usage",
                "--disable-background-networking",
                "--disable-default-apps",
                "--disable-extensions",
                "--disable-sync",
                "--metrics-recording-only",
                "--mute-audio",
                "--no-first-run",
                f"--user-data-dir={profile}",
                url,
            ],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.PIPE,
            text=True,
        )
        try:
            delivered = VERDICT_ARRIVED.wait(timeout=VERDICT_TIMEOUT_SECONDS)
        finally:
            browser.terminate()
            try:
                browser.wait(timeout=15)
            except subprocess.TimeoutExpired:
                browser.kill()
            server.shutdown()
            server.server_close()
            thread.join(timeout=5)

    if not delivered:
        raise SystemExit(
            "browser WASM JIT qualification reported no verdict within "
            f"{VERDICT_TIMEOUT_SECONDS}s; the worker never finished starting up"
        )

    expected = {
        "status": "qualified",
        "abi": "4",
        "solverResult": "15",
        "contributions": "3",
        "jacobians": "14",
        "stamps": str(EXPECTED_STAMPS),
    }
    mismatched = {
        key: VERDICT.get(key) for key, value in expected.items() if VERDICT.get(key) != value
    }
    if mismatched:
        raise SystemExit(
            "browser WASM JIT qualification failed "
            f"(expected {expected}, got {VERDICT}): {VERDICT.get('message', '')}"
        )

    nanoseconds = read_nanoseconds_per_stamp(VERDICT.get("nsPerStamp"))
    if nanoseconds > MAX_NANOSECONDS_PER_STAMP:
        raise SystemExit(
            f"browser WASM JIT stamp cost regressed: {nanoseconds:.1f} ns/stamp "
            f"exceeds the {MAX_NANOSECONDS_PER_STAMP} ns ceiling. A capability bound to a "
            "wasm-bindgen wrapper instead of a raw export, or a driver that stopped "
            "fusing, still computes the right answer and only shows up here."
        )
    print(
        "browser WASM JIT qualification passed: ABI 4, solver result 15, "
        f"3 contributions, 14 Jacobian entries, {nanoseconds:.1f} ns/stamp"
    )


if __name__ == "__main__":
    main()
