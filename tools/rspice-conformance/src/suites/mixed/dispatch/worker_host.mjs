// The browser worker under Node, for the mixed corpus's worker route.
//
//   node worker_host.mjs <web-dir>
//
// <web-dir> holds the application's `simulation-worker.js` beside the
// `pkg/` that wasm-bindgen generated for the worker image (the layout the
// browser serves). That script runs unchanged in a worker thread: this file
// gives it the few browser globals it calls (`self`, `postMessage`, a message
// listener, `fetch` of its own files) and posts it each request the way the
// application's main thread does, as a structured-clone `run` message with
// its buffers transferred. Frames on stdin and stdout are those of
// `worker_frames.mjs`.
//
// The scripts are ES modules named `.js`, which only a Node that detects
// module syntax loads from where they stand; so they run from a copy in a
// temporary directory that declares them modules, beside a link to the
// worker image itself.

import { Worker, isMainThread, parentPort, workerData } from "node:worker_threads";
import { cpSync, mkdtempSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { readFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { fileURLToPath, pathToFileURL } from "node:url";
import path from "node:path";

import { decodeFrame, encodeFrame, eventFrame, runMessage } from "./worker_frames.mjs";

if (isMainThread) {
  serve();
} else {
  await boot();
}

// The worker thread: browser globals, then the application's worker script.
async function boot() {
  globalThis.self = globalThis;
  globalThis.postMessage = (message, transfer) => parentPort.postMessage(message, transfer ?? []);
  globalThis.addEventListener = (type, listener) => {
    if (type === "message") {
      parentPort.on("message", (data) => listener({ data }));
    }
  };
  const networkFetch = globalThis.fetch;
  globalThis.fetch = async (input, init) => {
    const href = typeof input === "string" ? input : input instanceof URL ? input.href : input.url;
    const url = new URL(href);
    if (url.protocol !== "file:") {
      return networkFetch(input, init);
    }
    url.search = "";
    const bytes = await readFile(fileURLToPath(url));
    const type = url.pathname.endsWith(".wasm") ? "application/wasm" : "application/octet-stream";
    return new Response(bytes, { headers: { "Content-Type": type } });
  };
  await import(workerData.script);
}

// The main thread: the application's side of the worker conversation.
function serve() {
  const webDir = stage(path.resolve(process.argv[2] ?? "."));
  process.on("exit", () => rmSync(webDir, { recursive: true, force: true }));
  const script = pathToFileURL(path.join(webDir, "simulation-worker.js")).href;
  const worker = new Worker(new URL(import.meta.url), { workerData: { script } });

  worker.on("message", (message) => {
    const frame = eventFrame(message);
    if (frame) {
      process.stdout.write(frame);
    }
  });
  worker.on("error", (error) => {
    process.stdout.write(
      encodeFrame({ type: "error", id: 0, error: `worker thread failed: ${error?.stack ?? error}` }),
    );
    process.exitCode = 1;
  });
  // Stopping the worker once the runner closes its end is the ordinary way
  // out; any other exit is the worker's own failure.
  let closing = false;
  worker.on("exit", (code) => {
    process.exit(closing || code === 0 ? 0 : 1);
  });

  let pending = new Uint8Array(0);
  process.stdin.on("data", (chunk) => {
    const joined = new Uint8Array(pending.length + chunk.length);
    joined.set(pending);
    joined.set(chunk, pending.length);
    pending = joined;
    for (;;) {
      const frame = decodeFrame(pending);
      if (!frame) {
        break;
      }
      pending = pending.slice(frame.size);
      const { message, transfer } = runMessage(frame.header, frame.blobs);
      worker.postMessage(message, transfer);
    }
  });
  process.stdin.on("end", () => {
    closing = true;
    worker.terminate();
  });
}

// The worker's scripts, copied byte for byte into a directory that declares
// them ES modules, and a link to each WebAssembly image beside them.
function stage(webDir) {
  const staged = mkdtempSync(path.join(tmpdir(), "rspice-worker-"));
  writeFileSync(path.join(staged, "package.json"), '{"type":"module"}\n');
  for (const name of ["simulation-worker.js", "wasm-loader.js"]) {
    cpSync(path.join(webDir, name), path.join(staged, name));
  }
  const images = [];
  cpSync(path.join(webDir, "pkg"), path.join(staged, "pkg"), {
    recursive: true,
    filter: (source) => {
      if (source.endsWith(".wasm")) {
        images.push(source);
        return false;
      }
      return true;
    },
  });
  for (const image of images) {
    const link = path.join(staged, "pkg", path.relative(path.join(webDir, "pkg"), image));
    try {
      symlinkSync(image, link);
    } catch {
      cpSync(image, link);
    }
  }
  return staged;
}
