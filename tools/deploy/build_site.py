#!/usr/bin/env python3
r"""Build, verify, and assemble the public site into _site/.

The single definition of a site build — used verbatim by
.github/workflows/deploy-site.yml and runnable locally as a dry run:

    python3 tools/deploy/build_site.py [--skip-headless] [--out DIR]
    # Windows:  py tools\deploy\build_site.py --skip-headless

Stages:
  1. toolchain gate    — wasm-bindgen CLI must match Cargo.lock
  2. build             — rspice-ui (IDE, bin target) + rspice-wasm
                         (playground, lib) for wasm32, release
  3. assemble          — site/ verbatim + both pkg/ bundles + build.json
  4. static gates      — \0asm magic + wasm-bindgen export signature
  5. headless gate     — serve _site, load play/ in headless Chrome,
                         require "engine ready" AND a completed solve

Any failed gate exits non-zero; the workflow only publishes on success.
Pure stdlib, no third-party deps — runs the same on the Ubuntu CI runner
(python3) and on Windows (py). The local HTTP server uses this very
interpreter (sys.executable), sidestepping the Windows python-stub probe
the old shell script needed.
"""

import argparse
import datetime
import json
import os
import re
import shutil
import socket
import subprocess
import sys
import time
from pathlib import Path


def fail(msg):
    print("FAIL: " + msg, file=sys.stderr)
    sys.exit(1)


def run(cmd, **kw):
    """Run a command inheriting stdio; raise CalledProcessError on failure."""
    return subprocess.run(cmd, check=True, **kw)


def capture(cmd):
    return subprocess.run(cmd, capture_output=True, text=True, check=True).stdout.strip()


# ── 1 · toolchain gate ──────────────────────────────────────────────────
def locked_wasm_bindgen(root):
    """Version pinned in Cargo.lock — the first wasm-bindgen package entry,
    matching the shell `grep -A1 '^name = "wasm-bindgen"$' | grep '^version'`."""
    lines = (root / "Cargo.lock").read_text(encoding="utf-8").splitlines()
    for i, line in enumerate(lines):
        if line.strip() == 'name = "wasm-bindgen"':
            for nxt in lines[i + 1:i + 4]:
                m = re.match(r'version = "([^"]+)"', nxt.strip())
                if m:
                    return m.group(1)
    fail("could not find wasm-bindgen version in Cargo.lock")


def installed_wasm_bindgen():
    try:
        out = subprocess.run(["wasm-bindgen", "--version"],
                             capture_output=True, text=True, check=True).stdout
    except FileNotFoundError:
        fail("wasm-bindgen CLI not found on PATH "
             "(install the version Cargo.lock pins, or pass --skip-headless "
             "only skips the solve gate — the build still needs it)")
    return out.split()[1]  # "wasm-bindgen 0.2.114" -> "0.2.114"


# ── 4 · static gates ────────────────────────────────────────────────────
def gate_bundle(out, stem):
    wasm = out / (stem + "_bg.wasm")
    js = out / (stem + ".js")
    if wasm.read_bytes()[:4] != b"\x00asm":
        fail("%s is not a wasm module (magic %s)"
             % (wasm, wasm.read_bytes()[:4].hex()))
    if "export { initSync, __wbg_init as default }" not in js.read_text(encoding="utf-8"):
        fail(str(js) + " is missing the wasm-bindgen export signature")
    print("ok: %s (%d KiB wasm)" % (stem, wasm.stat().st_size // 1024))


# ── 5 · headless solve gate ─────────────────────────────────────────────
def find_chrome():
    env = os.environ.get("CHROME")
    if env and (Path(env).exists() or shutil.which(env)):
        return env
    for name in ("google-chrome", "chromium-browser", "chromium", "chrome"):
        found = shutil.which(name)
        if found:
            return found
    for path in (
        r"C:\Program Files\Google\Chrome\Application\chrome.exe",
        r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
        os.path.expandvars(r"%LOCALAPPDATA%\Google\Chrome\Application\chrome.exe"),
        r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
    ):
        if Path(path).exists():
            return path
    return None


def free_port():
    with socket.socket() as s:
        s.bind(("127.0.0.1", 0))
        return s.getsockname()[1]


def dump_dom(chrome, url, timeout=90):
    try:
        return subprocess.run(
            [chrome, "--headless=new", "--disable-gpu", "--no-sandbox",
             "--virtual-time-budget=20000", "--dump-dom", url],
            capture_output=True, text=True, timeout=timeout).stdout
    except subprocess.TimeoutExpired as e:
        return e.stdout.decode() if isinstance(e.stdout, bytes) else (e.stdout or "")


def headless_solve_gate(out):
    chrome = find_chrome()
    if not chrome:
        fail("no Chrome found for the headless gate "
             "(set CHROME or pass --skip-headless)")

    port = free_port()
    server = subprocess.Popen(
        [sys.executable, "-m", "http.server", str(port), "--directory", str(out)],
        stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    try:
        time.sleep(1)
        # play/index.html runs a transient on load; a healthy bundle yields
        # the "engine ready" badge and a "solved in" log line in the DOM.
        dom = dump_dom(chrome, "http://127.0.0.1:%d/play/" % port)
    finally:
        server.terminate()
        try:
            server.wait(timeout=5)
        except subprocess.TimeoutExpired:
            server.kill()

    if "engine ready" not in dom:
        m = re.search(r"module failed|init failed[^<]*", dom)
        if m:
            print(m.group(0), file=sys.stderr)
        fail("playground never reached 'engine ready'")
    if "solved in" not in dom:
        m = re.search(r"tran error[^<]*", dom)
        if m:
            print(m.group(0), file=sys.stderr)
        fail("playground loaded but the on-load transient did not solve")
    print("ok: headless solve — engine ready, transient completed")


def headless_ide_gate(out):
    chrome = find_chrome()
    if not chrome:
        fail("no Chrome found for the IDE headless gate "
             "(set CHROME or pass --skip-headless)")

    port = free_port()
    server = subprocess.Popen(
        [sys.executable, "-m", "http.server", str(port), "--directory", str(out)],
        stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    try:
        time.sleep(1)
        dom = dump_dom(chrome, "http://127.0.0.1:%d/ide/" % port)
    finally:
        server.terminate()
        try:
            server.wait(timeout=5)
        except subprocess.TimeoutExpired:
            server.kill()

    if "failed to load the RSpice module" in dom or "RSpice failed to start" in dom:
        m = re.search(r"(failed to load the RSpice module|RSpice failed to start)[^<]*", dom)
        if m:
            print(m.group(0), file=sys.stderr)
        fail("browser IDE reported a startup failure")
    if "id=\"rspice_loading\"" in dom:
        fail("browser IDE loaded but did not clear the loading overlay")
    print("ok: headless IDE boot - loading overlay cleared")


def main():
    ap = argparse.ArgumentParser(description="Build, verify, and assemble _site/.")
    ap.add_argument("--skip-headless", action="store_true",
                    help="skip the headless playground and IDE gates (no local Chrome)")
    ap.add_argument("--out", default="_site", help="output directory (default: _site)")
    args = ap.parse_args()

    root = Path(capture(["git", "rev-parse", "--show-toplevel"]))
    os.chdir(root)
    target = Path("target/wasm32-unknown-unknown/release")
    out = Path(args.out)

    # ── 1 · toolchain gate ──────────────────────────────────────────────
    locked = locked_wasm_bindgen(root)
    have = installed_wasm_bindgen()
    if locked != have:
        fail("wasm-bindgen CLI is %s but Cargo.lock pins %s" % (have, locked))
    print("ok: wasm-bindgen CLI %s matches Cargo.lock" % have)

    # ── 2 · build ───────────────────────────────────────────────────────
    print("building rspice-ui (browser IDE, bin target)...")
    run(["cargo", "build", "-p", "rspice-ui", "--bins",
         "--target", "wasm32-unknown-unknown", "--release"])
    print("building rspice-wasm (engine playground)...")
    run(["cargo", "build", "-p", "rspice-wasm", "--lib",
         "--target", "wasm32-unknown-unknown", "--release"])

    # ── 3 · assemble ────────────────────────────────────────────────────
    shutil.rmtree(out, ignore_errors=True)
    shutil.copytree("site", out)              # site/ verbatim
    (out / "README.md").unlink(missing_ok=True)

    run(["wasm-bindgen", str(target / "rspice-ui.wasm"),
         "--out-dir", str(out / "ide" / "pkg"),
         "--out-name", "rspice-ui", "--target", "web", "--no-typescript"])
    run(["wasm-bindgen", str(target / "rspice_wasm.wasm"),
         "--out-dir", str(out / "play" / "pkg"),
         "--out-name", "rspice_wasm", "--target", "web", "--no-typescript"])

    sha = capture(["git", "rev-parse", "HEAD"])
    built_utc = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    (out / "build.json").write_text(
        json.dumps({"source_sha": sha, "built_utc": built_utc, "wasm_bindgen": have},
                   separators=(",", ":")) + "\n", encoding="utf-8")

    # ── 4 · static gates ────────────────────────────────────────────────
    for stem in ("ide/pkg/rspice-ui", "play/pkg/rspice_wasm"):
        gate_bundle(out, stem)

    # ── 5 · headless solve gate ─────────────────────────────────────────
    if args.skip_headless:
        print("skipped: headless browser gates (--skip-headless)")
    else:
        headless_solve_gate(out)
        headless_ide_gate(out)

    print("site assembled at %s (source %s)" % (out, sha))


if __name__ == "__main__":
    main()
