#!/usr/bin/env python3
"""Host the browser worker in headless Chrome for the mixed corpus runner.

    RSPICE_WORKER_HOST="python3 tools/ci/browser_worker_host.py" \\
        rspice-mixed-corpus run --design C10 --worker crates/rspice-app/web

The corpus runner's worker route hands each prepared task to a worker host
over stdin and stdout, in the frames that
tools/rspice-conformance/src/suites/mixed/dispatch/worker_frames.mjs defines.
The default host runs the application's simulation-worker.js in a Node worker
thread; this one runs it as a module worker of a page in an isolated headless
Chrome profile, served from the web root by a local server. The page fetches
each request frame, posts it to the worker as the application does, and sends
back the frame each of the worker's messages makes. Nothing is judged here:
the runner reads the answers and classifies them as it does any route's.
"""

from __future__ import annotations

import functools
import http.server
import os
from pathlib import Path
import queue
import subprocess
import sys
import tempfile
import threading
import urllib.parse

from check_wasm_jit_browser import find_chromium, qualification_worker

FRAMES = (
    Path(__file__).resolve().parents[2]
    / "tools/rspice-conformance/src/suites/mixed/dispatch/worker_frames.mjs"
)

PAGE = (
    b'<!doctype html><meta charset="utf-8"><title>Worker host</title>'
    b'<script type="module" src="/worker-host.mjs"></script>'
)

SCRIPT = b"""
import { decodeFrame, encodeFrame, eventFrame, runMessage } from "/worker-frames.mjs";

const worker = new Worker(new URLSearchParams(location.search).get("worker"), { type: "module" });
let outbox = Promise.resolve();
const send = (frame) => {
  outbox = outbox.then(() => fetch("/event", { method: "POST", body: frame }));
};
worker.addEventListener("message", (event) => {
  const frame = eventFrame(event.data);
  if (frame) {
    send(frame);
  }
});
worker.addEventListener("error", (event) => {
  send(encodeFrame({ type: "error", id: 0, error: `worker failed: ${event.message}` }));
});
for (;;) {
  const response = await fetch("/next");
  if (response.status === 204) {
    continue;
  }
  if (response.status !== 200) {
    break;
  }
  const frame = decodeFrame(new Uint8Array(await response.arrayBuffer()));
  const { message, transfer } = runMessage(frame.header, frame.blobs);
  worker.postMessage(message, transfer);
}
"""


def read_frame(stream) -> bytes | None:
    """The next whole frame on `stream`, as its bytes, or None at its end."""

    def take(length: int) -> bytes | None:
        data = stream.read(length)
        return data if len(data) == length else None

    def u32() -> tuple[bytes, int] | None:
        data = take(4)
        return None if data is None else (data, int.from_bytes(data, "little"))

    parts = []
    length = u32()
    header = length and take(length[1])
    count = header is not None and u32()
    if not count:
        return None
    parts += [length[0], header, count[0]]
    for _ in range(count[1]):
        size = u32()
        blob = size and take(size[1])
        if blob is None or not size:
            return None
        parts += [size[0], blob]
    return b"".join(parts)


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit(f"usage: {sys.argv[0]} <web-dir>")
    root = Path(sys.argv[1]).resolve()
    worker = qualification_worker(root, "simulation-worker.js")
    frames = FRAMES.read_bytes()
    requests: queue.Queue[bytes | None] = queue.Queue()
    finished = threading.Event()
    output = threading.Lock()

    def forward_requests() -> None:
        while (frame := read_frame(sys.stdin.buffer)) is not None:
            requests.put(frame)
        # The runner closed its end: it wants nothing more.
        requests.put(None)
        finished.set()

    class Handler(http.server.SimpleHTTPRequestHandler):
        def log_message(self, *_args):
            pass

        def reply(self, status: int, body: bytes = b"", mime: str = "") -> None:
            self.send_response(status)
            if mime:
                self.send_header("Content-Type", mime)
            self.send_header("Content-Length", str(len(body)))
            self.send_header("Cache-Control", "no-store")
            self.end_headers()
            self.wfile.write(body)

        def do_GET(self):  # noqa: N802
            path = urllib.parse.urlparse(self.path).path
            if path == "/worker-host.html":
                return self.reply(200, PAGE, "text/html; charset=utf-8")
            if path == "/worker-host.mjs":
                return self.reply(200, SCRIPT, "text/javascript; charset=utf-8")
            if path == "/worker-frames.mjs":
                return self.reply(200, frames, "text/javascript; charset=utf-8")
            if path == "/next":
                try:
                    frame = requests.get(timeout=20)
                except queue.Empty:
                    return self.reply(204)
                if frame is None:
                    return self.reply(410)
                return self.reply(200, frame, "application/octet-stream")
            return super().do_GET()

        def do_POST(self):  # noqa: N802
            if self.path != "/event":
                return self.send_error(404)
            body = self.rfile.read(int(self.headers.get("Content-Length", "0")))
            with output:
                sys.stdout.buffer.write(body)
                sys.stdout.buffer.flush()
            self.reply(204)

    server = http.server.ThreadingHTTPServer(
        ("127.0.0.1", 0), functools.partial(Handler, directory=str(root))
    )
    threading.Thread(target=server.serve_forever, daemon=True).start()
    threading.Thread(target=forward_requests, daemon=True).start()
    query = urllib.parse.urlencode({"worker": "/" + worker.as_posix()})
    url = f"http://127.0.0.1:{server.server_port}/worker-host.html?{query}"
    try:
        # As the qualification harnesses do: Chromium's children can write the
        # profile back while it is deleted, and nothing is read from it.
        with tempfile.TemporaryDirectory(
            prefix="rspice-worker-browser-", ignore_cleanup_errors=True
        ) as profile:
            browser = subprocess.Popen(
                [find_chromium(), "--headless=new", "--disable-gpu",
                 "--disable-background-networking", "--disable-extensions",
                 "--disable-sync", "--no-first-run", "--no-default-browser-check",
                 f"--user-data-dir={profile}", url],
                stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
                creationflags=subprocess.CREATE_NO_WINDOW if os.name == "nt" else 0,
            )
            try:
                finished.wait()
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


if __name__ == "__main__":
    main()
