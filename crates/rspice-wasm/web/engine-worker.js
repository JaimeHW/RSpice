import init, {
  summarizeNetlist,
  runOperatingPointDocument,
  runAcAnalysisDocument,
  runTransientAnalysisDocument,
  runAuthoredDeckDocument,
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
    "code",
    "kind",
    "category",
    "retryable",
    "analysisId",
    "coordinateId",
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

/* The engine retains its results in WebAssembly memory and publishes bounded
   windows. The worker reads descriptors once and then transfers at most one
   budgeted window per result, so a long solve never becomes a second full
   JavaScript copy of itself. */
function windowPointBudget(metadata) {
  const perPoint = metadata.valuesPerPoint + metadata.signals.length;
  if (!(perPoint > 0)) {
    return metadata.pointCount;
  }
  return Math.max(1, Math.floor(metadata.maximumWindowValues / perPoint));
}

function readHandle(handle) {
  const metadata = handle.metadata();
  const results = metadata.results.map((summary) => {
    const detail = handle.resultMetadata(summary.index);
    let window = null;
    let truncated = false;
    if (detail.pointCount > 0) {
      const budget = windowPointBudget(detail);
      const count = Math.min(detail.pointCount, budget);
      truncated = count < detail.pointCount;
      window = handle.readWindow(summary.index, 0, count);
    }
    return { summary, metadata: detail, window, truncated };
  });
  return { metadata, results };
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
  const options = payload.options;

  try {
    await ensureReady();

    let result;
    switch (operation) {
      case "summary":
        result = summarizeNetlist(payload.source, options);
        break;
      case "op":
        result = readHandle(runOperatingPointDocument(payload.source, options));
        break;
      case "ac":
        result = readHandle(runAcAnalysisDocument(payload.source, payload.frequencies, options));
        break;
      case "tran":
        result = readHandle(
          runTransientAnalysisDocument(payload.source, payload.tstop, payload.hmax, options),
        );
        break;
      case "deck":
        result = readHandle(runAuthoredDeckDocument(payload.source, options));
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
