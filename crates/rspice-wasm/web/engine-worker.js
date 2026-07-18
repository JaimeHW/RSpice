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

function asErrorDetails(error) {
  const message = asErrorMessage(error);
  if (!error || typeof error !== "object") {
    return { message };
  }

  const details = error.details && typeof error.details === "object" ? error.details : error;
  const structured = { message };
  for (const field of [
    "kind",
    "category",
    "primarySource",
    "primaryLine",
    "relatedSource",
    "relatedLine",
    "firstStartupKind",
    "conflictingStartupKind",
    "iterations",
    "resource",
    "requested",
    "limit",
    "unresolvedOutputSymbols",
  ]) {
    if (Object.prototype.hasOwnProperty.call(details, field)) {
      structured[field] = details[field];
    }
  }
  return structured;
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
    const errorDetails = asErrorDetails(error);
    postMessage({
      type: "error",
      id,
      operation,
      error: errorDetails.message,
      errorDetails,
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
    const errorDetails = asErrorDetails(error);
    postMessage({
      type: "error",
      id: 0,
      operation: "init",
      error: errorDetails.message,
      errorDetails,
    });
  });
