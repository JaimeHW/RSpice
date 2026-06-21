import init, { runRspiceUiWorkerRequest } from "./pkg/rspice-ui.js";

let initPromise = null;

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

async function ensureReady() {
  if (!initPromise) {
    initPromise = init().catch((error) => {
      initPromise = null;
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
      const response = runRspiceUiWorkerRequest(message.request);
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
