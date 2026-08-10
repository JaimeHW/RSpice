#!/usr/bin/env python3
"""Run the optimized browser WASM JIT solver gate in headless Chromium."""

from __future__ import annotations

import argparse
import functools
import http.server
import pathlib
import shutil
import subprocess
import tempfile
import threading


class QuietHandler(http.server.SimpleHTTPRequestHandler):
    def log_message(self, _format: str, *_args: object) -> None:
        pass


def find_chromium() -> str:
    for candidate in (
        "google-chrome",
        "google-chrome-stable",
        "chromium",
        "chromium-browser",
    ):
        executable = shutil.which(candidate)
        if executable:
            return executable
    macos = pathlib.Path("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome")
    if macos.is_file():
        return str(macos)
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

    handler = functools.partial(QuietHandler, directory=str(web_root))
    server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), handler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    url = f"http://127.0.0.1:{server.server_port}/wasm-jit-qualification.html"
    try:
        with tempfile.TemporaryDirectory(prefix="rspice-wasm-jit-chrome-") as profile:
            completed = subprocess.run(
                [
                    find_chromium(),
                    "--headless=new",
                    "--no-sandbox",
                    "--disable-dev-shm-usage",
                    "--disable-background-networking",
                    "--disable-default-apps",
                    "--disable-extensions",
                    "--disable-sync",
                    "--metrics-recording-only",
                    "--mute-audio",
                    "--no-first-run",
                    f"--user-data-dir={profile}",
                    "--virtual-time-budget=35000",
                    "--dump-dom",
                    url,
                ],
                check=False,
                capture_output=True,
                text=True,
                timeout=90,
            )
    finally:
        server.shutdown()
        server.server_close()
        thread.join(timeout=5)

    evidence = completed.stdout
    required_evidence = (
        'data-rspice-wasm-jit-status="qualified"',
        'data-rspice-wasm-jit-abi="3"',
        'data-rspice-wasm-jit-solver-result="15"',
        "RSpice WASM JIT qualification passed.",
    )
    missing_evidence = [item for item in required_evidence if item not in evidence]
    if completed.returncode != 0 or missing_evidence:
        raise SystemExit(
            "browser WASM JIT qualification failed "
            f"(exit={completed.returncode}, missing={missing_evidence}):\n"
            f"{completed.stderr[-4000:]}\n{evidence[-4000:]}"
        )
    print("browser WASM JIT qualification passed: ABI 2, solver result 15")


if __name__ == "__main__":
    main()
