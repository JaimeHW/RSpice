#!/usr/bin/env python3
r"""Build, verify, and assemble the public site into _site/.

The single definition of a site build - used verbatim by
.github/workflows/deploy-site.yml and runnable locally as a dry run:

    python3 tools/deploy/build_site.py --site-source ../RSpice-Site/public
    # Windows:  py tools\deploy\build_site.py --site-source ..\RSpice-Site\public

Stages:
  1. toolchain gate    - wasm-bindgen CLI must match Cargo.lock
  2. build             - rspice-ui (IDE, bin target) + rspice-wasm
                         (playground, lib) for wasm32, release
  3. assemble          - RSpice-Site/public + client-owned browser shells,
                         generated wasm packages, and build.json
  4. static gates      - \0asm magic + wasm-bindgen export signature
  5. headless gate     - serve _site, load play/ and the IDE worker smoke
                         page in headless Chrome, require completed solves

Any failed gate exits non-zero; the workflow only publishes on success.
Pure stdlib, no third-party deps - runs the same on the Ubuntu CI runner
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


# 1. toolchain gate
def locked_wasm_bindgen(root):
    """Version pinned in Cargo.lock - the first wasm-bindgen package entry,
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
             "only skips the solve gate; the build still needs it)")
    return out.split()[1]  # "wasm-bindgen 0.2.114" -> "0.2.114"


# 4. static gates
def gate_bundle(out, stem):
    wasm = out / (stem + "_bg.wasm")
    js = out / (stem + ".js")
    if wasm.read_bytes()[:4] != b"\x00asm":
        fail("%s is not a wasm module (magic %s)"
             % (wasm, wasm.read_bytes()[:4].hex()))
    if "export { initSync, __wbg_init as default }" not in js.read_text(encoding="utf-8"):
        fail(str(js) + " is missing the wasm-bindgen export signature")
    print("ok: %s (%d KiB wasm)" % (stem, wasm.stat().st_size // 1024))


def require_tokens(path, source, tokens):
    for token in tokens:
        if token not in source:
            fail(str(path) + " is missing " + token)


def gate_ide_worker(out):
    index = out / "ide" / "index.html"
    worker = out / "ide" / "simulation-worker.js"
    if not worker.exists():
        fail(str(worker) + " is missing")

    index_source = index.read_text(encoding="utf-8")
    worker_source = worker.read_text(encoding="utf-8")
    require_tokens(index, index_source, (
        "__RSPICE_SIM_WORKER",
        "__RSPICE_SIM_WORKER_ERROR",
        'new URL("./simulation-worker.js", import.meta.url)',
        'addEventListener("error"',
        'addEventListener("messageerror"',
    ))
    if "innerHTML =" in index_source:
        fail(str(index) + " must render startup errors with text nodes, not innerHTML")
    require_tokens(worker, worker_source, (
        'from "./pkg/rspice-ui.js"',
        "runRspiceUiWorkerRequest",
        "responseTransferList(response)",
        "ArrayBuffer.isView(view)",
        "transferBuffers.add(view.buffer)",
        'id: message.id',
        'id: message.id ?? 0',
        'type: "ready"',
        'type: "result"',
        'type: "error"',
    ))
    print("ok: ide simulation worker")


def gate_ide_worker_sources(root):
    contract = root / "crates" / "rspice-ui" / "src" / "simulation" / "runner" / "worker_contract.rs"
    bridge = root / "crates" / "rspice-ui" / "src" / "simulation" / "runner" / "wasm_worker.rs"
    contract_source = contract.read_text(encoding="utf-8")
    bridge_source = bridge.read_text(encoding="utf-8")

    require_tokens(contract, contract_source, (
        "WorkerProgressSnapshot",
        "WorkerResponseTransport",
        "WorkerF64Series",
        "worker_response_transport_value",
        "worker_response_from_value",
        "validate_worker_response_id",
        "Float64Array",
        "emit_worker_progress_snapshot",
        'JsValue::from_str("progress")',
        "run_simulation_thread_with_progress_observer",
    ))
    require_tokens(bridge, bridge_source, (
        '"progress" => handle_progress_message',
        "active_progress",
        "WorkerProgressSnapshot",
        "worker_response_from_value",
        "validate_worker_response_id",
        "set_onmessageerror(Some",
        "drop_cached_worker",
        "__RSPICE_SIM_WORKER_ERROR",
    ))
    print("ok: ide worker source contract")


def gate_playground_worker(out):
    worker = out / "play" / "engine-worker.js"
    if not worker.exists():
        fail(str(worker) + " is missing")
    worker_source = worker.read_text(encoding="utf-8")
    require_tokens(worker, worker_source, (
        "summarizeNetlist",
        "runDcOperatingPoint",
        "runAcAnalysis",
        "runTransientAnalysis",
        'case "summary"',
        'case "op"',
        'case "ac"',
        'case "tran"',
        "runAcAnalysis(payload.source, payload.frequencies)",
        "let initPromise = null",
        "initPromise = init()",
        'type: "ready"',
        'type: "result"',
        'type: "error"',
    ))
    print("ok: playground engine worker")


# 5. headless solve gate
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


def validate_playground_dom(dom):
    if "worker ready" not in dom:
        m = re.search(r"module failed|init failed[^<]*", dom)
        if m:
            print(m.group(0), file=sys.stderr)
        fail("playground never reached 'worker ready'")
    if "solved in" not in dom:
        m = re.search(r"tran error[^<]*", dom)
        if m:
            print(m.group(0), file=sys.stderr)
        fail("playground loaded but the on-load transient did not solve")


def validate_ide_dom(dom):
    if 'id="rspice_canvas"' not in dom:
        fail("browser IDE route did not expose the RSpice canvas")
    if '<p class="err">' in dom:
        m = re.search(r'<p class="err">([^<]*)</p>', dom)
        if m:
            print(m.group(1), file=sys.stderr)
        fail("browser IDE rendered a startup error")


def validate_ide_worker_smoke_dom(dom):
    if "ide worker solved" in dom:
        return
    m = re.search(r"ide worker error:[^<]*", dom)
    if m:
        print(m.group(0), file=sys.stderr)
    fail("browser IDE worker loaded but did not complete the smoke solve")


def write_ide_worker_smoke_page(out):
    smoke = out / "ide-worker-smoke.html"
    smoke.write_text(
        """<!doctype html>
<html>
<body>
<pre id="ide-worker-smoke">starting</pre>
<script type="module">
const status = document.getElementById("ide-worker-smoke");
const setStatus = (text) => { status.textContent = text; };
let finished = false;
let sent = false;

function fail(message) {
  if (finished) return;
  finished = true;
  setStatus(`ide worker error: ${message}`);
  worker.terminate();
}

function complete(message) {
  if (finished) return;
  finished = true;
  setStatus(message);
  worker.terminate();
}

const worker = new Worker(new URL("./ide/simulation-worker.js", import.meta.url), {
  type: "module",
});

const request = {
  id: 314,
  request: { Config: "DcOp" },
  netlist: "* ide worker smoke\\nV1 in 0 1\\nR1 in 0 1k\\n.op\\n.end\\n",
  source_path: null,
};

function sendRequest() {
  if (sent) return;
  sent = true;
  setStatus("ide worker request sent");
  worker.postMessage({ type: "run", id: 314, request });
}

worker.addEventListener("message", (event) => {
  const message = event.data || {};
  if (message.type === "ready") {
    sendRequest();
    return;
  }
  if (message.type === "result" && message.id === 314) {
    const response = message.response && message.response.response;
    const outcome = response && response.outcome;
    if (outcome && Object.prototype.hasOwnProperty.call(outcome, "Success")) {
      complete("ide worker solved");
    } else {
      fail(JSON.stringify(outcome || message.response || message));
    }
    return;
  }
  if (message.type === "error") {
    fail(message.error || message.message || "unknown worker error");
  }
});

worker.addEventListener("error", (event) => {
  fail(event.message || "worker error");
});
worker.addEventListener("messageerror", () => {
  fail("worker messageerror");
});
setTimeout(() => {
  if (!finished) fail("timeout");
}, 15000);
</script>
</body>
</html>
""",
        encoding="utf-8",
    )
    return smoke


def chrome_headless_args(chrome, url, width=1280, height=900):
    return [
        chrome,
        "--headless=new",
        "--no-sandbox",
        "--enable-unsafe-webgpu",
        "--ignore-gpu-blocklist",
        "--enable-features=Vulkan",
        "--use-vulkan=swiftshader",
        "--window-size=%d,%d" % (width, height),
        "--virtual-time-budget=20000",
        "--dump-dom",
        url,
    ]


def dump_chrome_dom(chrome, url, width=1280, height=900):
    try:
        return subprocess.run(
            chrome_headless_args(chrome, url, width, height),
            capture_output=True, text=True, timeout=90).stdout
    except subprocess.TimeoutExpired as e:
        return e.stdout.decode() if isinstance(e.stdout, bytes) else (e.stdout or "")


def headless_solve_gate(out):
    chrome = find_chrome()
    if not chrome:
        fail("no Chrome found for the headless gate "
             "(set CHROME or pass --skip-headless)")

    port = free_port()
    smoke_page = write_ide_worker_smoke_page(out)
    server = subprocess.Popen(
        [sys.executable, "-m", "http.server", str(port), "--directory", str(out)],
        stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    try:
        time.sleep(1)
        # play/index.html runs a transient on load; a healthy bundle yields
        # the "worker ready" badge and a "solved in" log line in the DOM.
        play_url = "http://127.0.0.1:%d/play/" % port
        ide_url = "http://127.0.0.1:%d/ide/" % port
        viewports = (
            ("desktop", 1280, 900),
            ("tablet", 820, 1180),
            ("phone", 390, 844),
        )
        viewport_doms = [
            (
                label,
                dump_chrome_dom(chrome, play_url, width, height),
                dump_chrome_dom(chrome, ide_url, width, height),
            )
            for label, width, height in viewports
        ]
        ide_smoke_dom = dump_chrome_dom(
            chrome, "http://127.0.0.1:%d/ide-worker-smoke.html" % port
        )
    finally:
        server.terminate()
        try:
            server.wait(timeout=5)
        except subprocess.TimeoutExpired:
            server.kill()
        smoke_page.unlink(missing_ok=True)

    for label, play_dom, ide_dom in viewport_doms:
        validate_playground_dom(play_dom)
        print("ok: headless solve (%s viewport) - worker ready, transient completed" % label)
        validate_ide_dom(ide_dom)
        print("ok: headless IDE (%s viewport) - route loaded without startup error" % label)
    validate_ide_worker_smoke_dom(ide_smoke_dom)
    print("ok: headless IDE worker - request solved")


def validate_site_source(site_source):
    """Validate the cross-repository static-source boundary before building."""
    if not site_source.is_dir():
        fail("site source directory does not exist: %s" % site_source)
    for required in ("index.html", "404.html", "assets"):
        if not (site_source / required).exists():
            fail("site source is missing required path: %s" % (site_source / required))
    for reserved in ("ide", "play"):
        if (site_source / reserved).exists():
            fail(
                "site source must not provide the client-owned runtime route '%s/'"
                % reserved
            )
    for path in site_source.rglob("*"):
        if path.is_symlink():
            fail("site source must not contain symlinks: %s" % path)


def site_source_revision(site_source):
    """Return the exact source commit for the standalone site checkout."""
    try:
        revision = capture(["git", "-C", str(site_source), "rev-parse", "HEAD"])
    except subprocess.CalledProcessError:
        revision = os.environ.get("RSPICE_SITE_SOURCE_SHA", "")
    if not re.fullmatch(r"[0-9a-f]{40}", revision):
        fail(
            "site source is not in a Git checkout and RSPICE_SITE_SOURCE_SHA "
            "is not a full commit SHA"
        )
    return revision


def copy_runtime_shell(source, destination, names):
    destination.mkdir(parents=True)
    for name in names:
        path = source / name
        if not path.is_file():
            fail("browser runtime source is missing: %s" % path)
        shutil.copy2(path, destination / name)


def assemble_site_sources(root, site_source, out):
    """Combine static site source with client-owned browser runtime shells."""
    shutil.rmtree(out, ignore_errors=True)
    shutil.copytree(site_source, out)
    copy_runtime_shell(
        root / "crates" / "rspice-ui" / "web",
        out / "ide",
        ("index.html", "simulation-worker.js"),
    )
    copy_runtime_shell(
        root / "crates" / "rspice-wasm" / "web",
        out / "play",
        ("index.html", "engine-worker.js"),
    )


def main():
    ap = argparse.ArgumentParser(description="Build, verify, and assemble _site/.")
    ap.add_argument("--skip-headless", action="store_true",
                    help="skip the headless playground solve gate (no local Chrome)")
    ap.add_argument("--out", default="_site", help="output directory (default: _site)")
    ap.add_argument(
        "--site-source",
        default="_site-source/public",
        help="RSpice-Site public directory (default: _site-source/public)",
    )
    args = ap.parse_args()

    root = Path(capture(["git", "rev-parse", "--show-toplevel"]))
    os.chdir(root)
    target = Path("target/wasm32-unknown-unknown/release")
    out = Path(args.out)
    site_source = Path(args.site_source).resolve()
    validate_site_source(site_source)
    site_sha = site_source_revision(site_source)

    # 1. toolchain gate
    locked = locked_wasm_bindgen(root)
    have = installed_wasm_bindgen()
    if locked != have:
        fail("wasm-bindgen CLI is %s but Cargo.lock pins %s" % (have, locked))
    print("ok: wasm-bindgen CLI %s matches Cargo.lock" % have)

    # 2. build
    print("building rspice-ui (browser IDE, bin target)...")
    run(["cargo", "build", "--locked", "-p", "rspice-ui", "--bins",
         "--target", "wasm32-unknown-unknown", "--release"])
    print("building rspice-wasm (engine playground)...")
    run(["cargo", "build", "--locked", "-p", "rspice-wasm", "--lib",
         "--target", "wasm32-unknown-unknown", "--release"])

    # 3. assemble
    assemble_site_sources(root, site_source, out)

    run(["wasm-bindgen", str(target / "rspice-ui.wasm"),
         "--out-dir", str(out / "ide" / "pkg"),
         "--out-name", "rspice-ui", "--target", "web", "--no-typescript"])
    run(["wasm-bindgen", str(target / "rspice_wasm.wasm"),
         "--out-dir", str(out / "play" / "pkg"),
         "--out-name", "rspice_wasm", "--target", "web", "--no-typescript"])

    client_sha = capture(["git", "rev-parse", "HEAD"])
    built_utc = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    (out / "build.json").write_text(
        json.dumps(
            {
                # Keep source_sha as a compatibility alias for existing monitors.
                "source_sha": client_sha,
                "client_source_sha": client_sha,
                "site_source_sha": site_sha,
                "built_utc": built_utc,
                "wasm_bindgen": have,
            },
            separators=(",", ":"),
        )
        + "\n",
        encoding="utf-8",
    )

    # 4. static gates
    for stem in ("ide/pkg/rspice-ui", "play/pkg/rspice_wasm"):
        gate_bundle(out, stem)
    gate_ide_worker(out)
    gate_ide_worker_sources(root)
    gate_playground_worker(out)

    # 5. headless solve gate
    if args.skip_headless:
        print("skipped: headless solve gate (--skip-headless)")
    else:
        headless_solve_gate(out)

    print(
        "site assembled at %s (client %s, site %s)"
        % (out, client_sha, site_sha)
    )


if __name__ == "__main__":
    main()
