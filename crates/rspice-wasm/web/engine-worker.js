import init, {
  summarizeNetlist,
  runDcOperatingPoint,
  runAcAnalysis,
  runTransientAnalysis,
} from "./pkg/rspice_wasm.js";

let initPromise = null;

function asErrorMessage(error) {
  return error instanceof Error ? error.message : String(error);
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

async function handleRequest(message) {
  const { id, operation, payload } = message;
  const start = performance.now();

  try {
    await ensureReady();

    let result;
    switch (operation) {
      case "summary":
        result = summarizeNetlist(payload.source);
        break;
      case "op":
        result = runDcOperatingPoint(payload.source);
        break;
      case "ac":
        result = runAcAnalysis(payload.source, payload.frequencies);
        break;
      case "tran":
        result = runTransientAnalysis(payload.source, payload.tstop, payload.hmax);
        break;
      default:
        throw new Error(`unknown engine operation '${operation}'`);
    }

    postMessage({
      type: "result",
      id,
      operation,
      elapsedMs: performance.now() - start,
      result,
    });
  } catch (error) {
    postMessage({
      type: "error",
      id,
      operation,
      error: asErrorMessage(error),
    });
  }
}

self.addEventListener("message", (event) => {
  const message = event.data || {};
  if (message.type === "run") {
    void handleRequest(message);
  }
});

ensureReady()
  .then(() => {
    postMessage({ type: "ready" });
  })
  .catch((error) => {
    postMessage({ type: "error", id: 0, operation: "init", error: asErrorMessage(error) });
  });
