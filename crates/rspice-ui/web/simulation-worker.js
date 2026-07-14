const workerUrl = new URL(import.meta.url);
const immutableReleaseAsset = /\/assets\/[0-9a-f]{64}\/simulation-worker\.js$/.test(
  workerUrl.pathname,
);
const developmentAssetVersion =
  workerUrl.searchParams.get("v") || `worker-${Date.now()}`;

function executableAsset(name) {
  const path = immutableReleaseAsset ? `./${name}` : `./pkg/${name}`;
  const url = new URL(path, import.meta.url);
  if (!immutableReleaseAsset) {
    url.searchParams.set("v", developmentAssetVersion);
  }
  return url;
}

let initPromise = null;
let runWorkerRequest = null;

function asErrorMessage(error) {
  return error instanceof Error ? error.message : String(error);
}

function responseTransferList(response) {
  if (!response || response.protocolVersion !== 2 || !Array.isArray(response.buffers)) {
    return [];
  }

  const transferBuffers = new Set();
  for (const view of response.buffers) {
    if (ArrayBuffer.isView(view) && view.buffer instanceof ArrayBuffer) {
      transferBuffers.add(view.buffer);
    }
  }
  return Array.from(transferBuffers);
}

async function initializeWorkerModule() {
  const module = await import(executableAsset("rspice-ui.js").href);
  const wasmModule = executableAsset("rspice-ui_bg.wasm");
  await module.default({ module_or_path: wasmModule });
  if (typeof module.runRspiceUiWorkerRequest !== "function") {
    throw new Error("RSpice worker package is missing its request executor.");
  }
  runWorkerRequest = module.runRspiceUiWorkerRequest;
}

async function ensureReady() {
  if (!initPromise) {
    initPromise = initializeWorkerModule().catch((error) => {
      initPromise = null;
      runWorkerRequest = null;
      throw error;
    });
  }
  await initPromise;
}

self.addEventListener("message", (event) => {
  const message = event.data || {};
  if (message.type !== "run") {
    return;
  }

  void (async () => {
    try {
      await ensureReady();
      const response = runWorkerRequest(message.request);
      postMessage(
        { type: "result", id: message.id, response },
        responseTransferList(response),
      );
    } catch (error) {
      postMessage({
        type: "error",
        id: message.id ?? 0,
        error: asErrorMessage(error),
      });
    }
  })();
});

ensureReady()
  .then(() => {
    postMessage({ type: "ready" });
  })
  .catch((error) => {
    postMessage({ type: "error", id: 0, error: asErrorMessage(error) });
  });
