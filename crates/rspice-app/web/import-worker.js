// The browser's import worker: the page starts one when an import starts and lets it go when the
// import ends, so no reader of a result file's format loads with the page. It reads the picked
// file here and asks the page to relay what it keeps to the results store.
const workerUrl = new URL(import.meta.url);
const immutableReleaseAsset = /\/assets\/[0-9a-f]{64}\/import-worker\.js$/.test(
  workerUrl.pathname,
);
const developmentAssetVersion =
  workerUrl.searchParams.get("v") || `worker-${Date.now()}`;

function executableAsset(name) {
  const path = immutableReleaseAsset || name === "wasm-loader.js" ? `./${name}` : `./pkg/${name}`;
  const url = new URL(path, import.meta.url);
  if (!immutableReleaseAsset) {
    url.searchParams.set("v", developmentAssetVersion);
  }
  return url;
}

function asErrorMessage(error) {
  return error instanceof Error ? error.message : String(error);
}

async function loadImporter() {
  const [module, { loadWasm }] = await Promise.all([
    import(executableAsset("rspice-ui-import-worker.js").href),
    import(executableAsset("wasm-loader.js").href),
  ]);
  const wasmModule = executableAsset(
    immutableReleaseAsset
      ? "rspice-ui-import-worker_bg.wasm.gz"
      : "rspice-ui-import-worker_bg.wasm",
  );
  await module.default({ module_or_path: await loadWasm(wasmModule) });
  if (typeof module.importRspiceResultFile !== "function") {
    throw new Error("RSpice import worker package is missing its importer.");
  }
  return module;
}

// Store requests in flight through the page, by their number.
const pending = new Map();
let nextRequest = 1;

// Ask the page to relay one results-store request; the rows' buffers move with it.
function store(request, buffers) {
  const id = nextRequest++;
  const views = Array.from(buffers);
  const transfer = Array.from(
    new Set(
      views
        .filter((view) => ArrayBuffer.isView(view) && view.buffer instanceof ArrayBuffer)
        .map((view) => view.buffer),
    ),
  );
  return new Promise((resolve, reject) => {
    pending.set(id, { resolve, reject });
    postMessage({ type: "store", id, request, buffers: views }, transfer);
  });
}

self.addEventListener("message", (event) => {
  const message = event.data || {};
  if (message.type === "store-result" || message.type === "store-error") {
    const waiting = pending.get(message.id);
    pending.delete(message.id);
    if (!waiting) {
      return;
    }
    if (message.type === "store-result") {
      waiting.resolve({ response: message.response, buffers: message.buffers || [] });
    } else {
      waiting.reject(new Error(message.error || "The results store refused the import."));
    }
    return;
  }
  if (message.type === "import") {
    void (async () => {
      try {
        if (!(message.file instanceof Blob) || typeof message.policy !== "string") {
          throw new Error("Malformed RSpice import request.");
        }
        const module = await loadImporter();
        const outcome = await module.importRspiceResultFile(
          message.file,
          String(message.name || ""),
          message.policy,
          store,
        );
        postMessage({ type: "imported", outcome });
      } catch (error) {
        postMessage({ type: "failed", error: asErrorMessage(error) });
      }
    })();
  }
});
