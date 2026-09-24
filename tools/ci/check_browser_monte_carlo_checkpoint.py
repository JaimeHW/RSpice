#!/usr/bin/env python3
"""Qualify Monte Carlo continuation through the shipping browser worker.

Uses an isolated headless Chrome profile and a local server. Requires the normal
worker wasm-bindgen package in --web-root; no interactive UI image is needed.
"""

from __future__ import annotations

import argparse
import functools
import hashlib
import http.server
import json
import os
from pathlib import Path
import subprocess
import tempfile
import threading
import urllib.parse

from check_wasm_jit_browser import find_chromium, qualification_assets, qualification_worker


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--web-root", type=Path, default=Path("crates/rspice-app/web"))
    parser.add_argument("--worker-path", default="simulation-worker.js")
    parser.add_argument("--output", type=Path, help="Retain the verdict and tested asset hashes")
    args = parser.parse_args()
    root = args.web_root.resolve()
    worker = qualification_worker(root, args.worker_path)
    script = Path(__file__).with_name("monte-carlo-checkpoint-qualification.mjs").read_bytes()
    verdict = {}
    delivered = threading.Event()

    class Handler(http.server.SimpleHTTPRequestHandler):
        def log_message(self, *_args):
            pass

        def do_GET(self):  # noqa: N802
            path = urllib.parse.urlparse(self.path).path
            if path == "/mc-checkpoint.html":
                body = b'<!doctype html><meta charset="utf-8"><title>Monte Carlo checkpoint qualification</title><script type="module" src="/mc-checkpoint.mjs"></script>'
                mime = "text/html; charset=utf-8"
            elif path == "/mc-checkpoint.mjs":
                body, mime = script, "text/javascript; charset=utf-8"
            else:
                return super().do_GET()
            self.send_response(200)
            self.send_header("Content-Type", mime)
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)

        def do_POST(self):  # noqa: N802
            if self.path != "/mc-checkpoint-verdict":
                return self.send_error(404)
            length = int(self.headers.get("Content-Length", "0"))
            if not 0 < length <= 16384:
                return self.send_error(413)
            result = json.loads(self.rfile.read(length))
            if not isinstance(result, dict) or delivered.is_set():
                return self.send_error(400)
            verdict.update(result)
            self.send_response(204)
            self.end_headers()
            delivered.set()

    server = http.server.ThreadingHTTPServer(
        ("127.0.0.1", 0), functools.partial(Handler, directory=str(root))
    )
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    query = urllib.parse.urlencode({"worker": "/" + worker.as_posix()})
    url = f"http://127.0.0.1:{server.server_port}/mc-checkpoint.html?{query}"
    try:
        with tempfile.TemporaryDirectory(prefix="rspice-mc-browser-") as profile:
            browser = subprocess.Popen(
                [find_chromium(), "--headless=new", "--disable-gpu",
                 "--disable-background-networking", "--disable-extensions",
                 "--disable-sync", "--no-first-run", "--no-default-browser-check",
                 f"--user-data-dir={profile}", url],
                stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
                creationflags=subprocess.CREATE_NO_WINDOW if os.name == "nt" else 0,
            )
            try:
                if not delivered.wait(timeout=240):
                    verdict.update(status="failed", error="Browser worker check timed out")
            finally:
                browser.terminate()
                try:
                    browser.wait(timeout=15)
                except subprocess.TimeoutExpired:
                    browser.kill()
                    browser.wait()
    finally:
        server.shutdown()
        server.server_close()
        thread.join(timeout=5)

    assets = {"worker": root / worker, **qualification_assets(root / worker)}
    verdict["assets"] = {
        name: {"path": str(path), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}
        for name, path in assets.items()
    }
    report = json.dumps(verdict, indent=2)
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(report + "\n", encoding="utf-8")
    print(report)
    if verdict.get("status") != "passed":
        raise SystemExit(1)


if __name__ == "__main__":
    main()
