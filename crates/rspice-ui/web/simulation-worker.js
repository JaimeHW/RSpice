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
let runVerilogACompileRequest = null;
let runHardcopyRequest = null;
let runModelImportRequest = null;
let runPdkImportRequest = null;
let primaryWasmExports = null;
let workerModule = null;
const wasmJitModelCache = new Map();
const WASM_JIT_CACHE_MAX_MODELS = 64;
const WASM_JIT_CACHE_MAX_BYTES = 64 * 1024 * 1024;
const WASM_JIT_MODEL_MAX_BYTES = 32 * 1024 * 1024;
const WASM_JIT_IDENTITY = /^[0-9a-f]{64}$/;
// Raw WebAssembly exports of the primary module, bound straight into every
// generated module. Passing the wasm-bindgen JavaScript wrappers instead would
// put a JS frame -- and, for the descriptor helpers, BigInt marshalling of
// their i64 arguments -- between a model and its implementation on a path that
// runs thousands of times per device evaluation.
const WASM_JIT_RAW_CAPABILITY_EXPORTS = [
  "rspice_ui_wasm_jit_eval_op_v1",
  "rspice_ui_wasm_jit_eval_op_slice_v1",
  "rspice_ui_wasm_jit_math1_v1",
  "rspice_ui_wasm_jit_math2_v1",
];
const WASM_JIT_VALUE_EXPORT = /^rspice_wasm_jit_value_[0-9a-f]{8}$/;
// Whole-model drivers. A module carries these only when the shared
// contribution-ordering rule allows fusing, so they are validated when present
// and simply absent otherwise.
const WASM_JIT_KERNEL_EXPORTS = {
  evaluationKernelExport: "rspice_wasm_jit_eval_kernel",
  stampKernelExport: "rspice_wasm_jit_stamp_kernel",
};
let wasmJitCacheBytes = 0;
let wasmJitCapability = {
  available: false,
  reason: "WASM JIT architecture qualification has not run.",
};
const WORKER_PROTOCOL_VERSION = 14;
const WORKER_REQUEST_PROTOCOL_VERSION = 8;
const HARDCOPY_PROTOCOL_VERSION = 1;

function asErrorMessage(error) {
  return error instanceof Error ? error.message : String(error);
}

function protocolResponseTransferList(response, expectedProtocolVersion) {
  if (
    !response ||
    response.protocolVersion !== expectedProtocolVersion ||
    !Array.isArray(response.buffers)
  ) {
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

function responseTransferList(response) {
  return protocolResponseTransferList(response, WORKER_PROTOCOL_VERSION);
}

function hardcopyResponseTransferList(response) {
  return protocolResponseTransferList(response, HARDCOPY_PROTOCOL_VERSION);
}

function modelImportResponseTransferList(response) {
  const view = response?.libraryBytes;
  return ArrayBuffer.isView(view) && view.buffer instanceof ArrayBuffer
    ? [view.buffer]
    : [];
}

function pdkImportResponseTransferList(response) {
  const view = response?.payloadBytes;
  return ArrayBuffer.isView(view) && view.buffer instanceof ArrayBuffer
    ? [view.buffer]
    : [];
}

/// Build the capability record every generated module is instantiated against.
function wasmJitImports(wasmExports) {
  return {
    memory: wasmExports.memory,
    eval_op_v1: wasmExports.rspice_ui_wasm_jit_eval_op_v1,
    eval_op_slice_v1: wasmExports.rspice_ui_wasm_jit_eval_op_slice_v1,
    math1_v1: wasmExports.rspice_ui_wasm_jit_math1_v1,
    math2_v1: wasmExports.rspice_ui_wasm_jit_math2_v1,
  };
}

async function qualifyWasmJitArchitecture(module, wasmExports) {
  const requiredFunctions = [
    "rspiceUiWasmJitProbeModule",
    "rspiceUiWasmJitAbiVersion",
    "rspiceUiWasmJitEmitterVersion",
    "prepareRspiceUiWasmJitProbe",
    "finishRspiceUiWasmJitProbe",
    "rspiceUiWasmJitSolverProbeArtifact",
    "rspiceUiWasmJitRunSolverProbe",
    "rspiceUiWasmJitKernelProbeArtifact",
    "rspiceUiWasmJitRunKernelProbe",
  ];
  for (const name of requiredFunctions) {
    if (typeof module[name] !== "function") {
      return {
        available: false,
        reason: `RSpice worker package is missing ${name}.`,
      };
    }
  }
  for (const name of WASM_JIT_RAW_CAPABILITY_EXPORTS) {
    if (typeof wasmExports?.[name] !== "function") {
      return {
        available: false,
        reason: `RSpice worker did not expose raw capability export ${name}.`,
      };
    }
  }
  if (!(wasmExports?.memory instanceof WebAssembly.Memory)) {
    return {
      available: false,
      reason: "RSpice worker did not expose its WebAssembly memory.",
    };
  }
  if (typeof WebAssembly.compile !== "function") {
    return {
      available: false,
      reason: "This browser does not provide WebAssembly.compile.",
    };
  }

  try {
    const emitted = module.rspiceUiWasmJitProbeModule();
    const bytes = Uint8Array.from(emitted);
    const compileStarted = performance.now();
    const compiled = await WebAssembly.compile(bytes);
    const compileMs = performance.now() - compileStarted;
    const instantiateStarted = performance.now();
    const instance = await WebAssembly.instantiate(compiled, {
      rspice_jit: wasmJitImports(wasmExports),
    });
    const instantiateMs = performance.now() - instantiateStarted;
    const probe = instance.exports.rspice_wasm_jit_probe;
    if (typeof probe !== "function") {
      throw new Error("verified probe module is missing its entrypoint");
    }
    const frameOffset = module.prepareRspiceUiWasmJitProbe();
    const status = probe(frameOffset);
    const result = module.finishRspiceUiWasmJitProbe(frameOffset, status);
    const solverArtifact = module.rspiceUiWasmJitSolverProbeArtifact();
    await installWasmJitArtifact(module, solverArtifact);
    const solverResult = module.rspiceUiWasmJitRunSolverProbe();
    if (solverResult !== 15) {
      throw new Error(`WASM JIT solver probe produced ${solverResult}, expected 15.`);
    }
    // A second model, installed the same way, whose contributions and
    // derivatives are checked bit for bit inside the worker and whose stamp
    // cost is timed. It carries the transcendentals, the extremum and the
    // multi-entry Jacobian the solver probe above does not.
    const kernelArtifact = module.rspiceUiWasmJitKernelProbeArtifact();
    await installWasmJitArtifact(module, kernelArtifact);
    const kernel = module.rspiceUiWasmJitRunKernelProbe();
    if (!(kernel?.nanosecondsPerStamp > 0)) {
      throw new Error("WASM JIT kernel probe reported no measurable stamp cost.");
    }
    return {
      available: true,
      abiVersion: module.rspiceUiWasmJitAbiVersion(),
      moduleBytes: bytes.byteLength,
      compileMs,
      instantiateMs,
      result,
      solverResult,
      kernel,
    };
  } catch (error) {
    return {
      available: false,
      reason: asErrorMessage(error),
    };
  }
}

async function installWasmJitArtifact(module, artifact) {
  if (!(primaryWasmExports?.memory instanceof WebAssembly.Memory)) {
    throw new Error("RSpice worker memory is unavailable for model JIT installation.");
  }
  if (!artifact || typeof artifact !== "object") {
    throw new Error("WASM JIT model artifact is missing.");
  }
  if (!WASM_JIT_IDENTITY.test(artifact.cacheKey || "")) {
    throw new Error("WASM JIT model artifact has an invalid cache identity.");
  }
  if (!WASM_JIT_IDENTITY.test(artifact.digest || "")) {
    throw new Error("WASM JIT model artifact has an invalid module digest.");
  }
  if (artifact.abiVersion !== module.rspiceUiWasmJitAbiVersion()) {
    throw new Error("WASM JIT model artifact has an incompatible ABI version.");
  }
  if (artifact.emitterVersion !== module.rspiceUiWasmJitEmitterVersion()) {
    throw new Error("WASM JIT model artifact has an incompatible emitter version.");
  }
  if (!Array.isArray(artifact.valueExports)) {
    throw new Error("WASM JIT model artifact has no verified value manifest.");
  }
  const uniqueValueExports = new Set(artifact.valueExports);
  if (
    uniqueValueExports.size !== artifact.valueExports.length ||
    artifact.valueExports.some((name) => !WASM_JIT_VALUE_EXPORT.test(name))
  ) {
    throw new Error("WASM JIT model artifact has an invalid value manifest.");
  }
  if (artifact.assignmentExport !== "rspice_wasm_jit_assign") {
    throw new Error("WASM JIT model artifact has an invalid assignment export.");
  }
  if (
    artifact.postAssignmentExport != null &&
    artifact.postAssignmentExport !== "rspice_wasm_jit_post_assign"
  ) {
    throw new Error("WASM JIT model artifact has an invalid post-assignment export.");
  }
  for (const [field, expected] of Object.entries(WASM_JIT_KERNEL_EXPORTS)) {
    if (artifact[field] != null && artifact[field] !== expected) {
      throw new Error(`WASM JIT model artifact has an invalid ${field}.`);
    }
  }
  const cached = wasmJitModelCache.get(artifact.cacheKey);
  if (cached) {
    wasmJitModelCache.delete(artifact.cacheKey);
    wasmJitModelCache.set(artifact.cacheKey, cached);
    return cached;
  }
  const bytes = Uint8Array.from(artifact.moduleBytes || []);
  if (bytes.byteLength === 0 || bytes.byteLength > WASM_JIT_MODEL_MAX_BYTES) {
    throw new Error(
      `WASM JIT model artifact is ${bytes.byteLength} bytes, outside the verified model budget.`,
    );
  }
  const compiled = await WebAssembly.compile(bytes);
  const instance = await WebAssembly.instantiate(compiled, {
    rspice_jit: wasmJitImports(primaryWasmExports),
  });
  for (const name of artifact.valueExports || []) {
    if (typeof instance.exports[name] !== "function") {
      throw new Error(`WASM JIT model is missing verified value export ${name}.`);
    }
  }
  if (typeof instance.exports[artifact.assignmentExport] !== "function") {
    throw new Error("WASM JIT model is missing its assignment kernel.");
  }
  if (
    artifact.postAssignmentExport &&
    typeof instance.exports[artifact.postAssignmentExport] !== "function"
  ) {
    throw new Error("WASM JIT model is missing its post-assignment kernel.");
  }
  for (const field of Object.keys(WASM_JIT_KERNEL_EXPORTS)) {
    if (artifact[field] && typeof instance.exports[artifact[field]] !== "function") {
      throw new Error(`WASM JIT model declares ${field} but does not export it.`);
    }
  }
  const installed = {
    artifact: { ...artifact, moduleBytes: undefined },
    compiled,
    instance,
    moduleBytes: bytes.byteLength,
  };
  while (
    wasmJitModelCache.size >= WASM_JIT_CACHE_MAX_MODELS ||
    wasmJitCacheBytes + bytes.byteLength > WASM_JIT_CACHE_MAX_BYTES
  ) {
    const oldestKey = wasmJitModelCache.keys().next().value;
    if (oldestKey === undefined) {
      break;
    }
    const oldest = wasmJitModelCache.get(oldestKey);
    wasmJitModelCache.delete(oldestKey);
    wasmJitCacheBytes -= oldest.moduleBytes;
  }
  wasmJitModelCache.set(artifact.cacheKey, installed);
  wasmJitCacheBytes += bytes.byteLength;
  return installed;
}

async function installWasmJitModel(module, response) {
  const artifact = response?.wasmJitArtifact;
  if (!artifact) {
    if (response?.wasmJitError) {
      wasmJitCapability = {
        ...wasmJitCapability,
        lastModelError: String(response.wasmJitError),
      };
      console.warn(`RSpice WASM JIT model qualification failed: ${response.wasmJitError}`);
    }
    return null;
  }
  return installWasmJitArtifact(module, artifact);
}

async function prepareWasmJitSimulationRequest(module, request) {
  const preparation = module.prepareRspiceUiWasmJitRequest(request);
  const candidateToken = preparation?.dispatchToken;
  try {
    if (
      !preparation ||
      !Number.isInteger(candidateToken) ||
      candidateToken <= 0 ||
      !Array.isArray(preparation.artifacts) ||
      !Array.isArray(preparation.errors)
    ) {
      throw new Error("RSpice worker returned malformed WASM JIT request preparation.");
    }
    const requiredBytes = preparation.artifacts.reduce((total, artifact) => {
      const bytes = artifact?.moduleBytes?.length;
      if (!Number.isSafeInteger(bytes) || bytes < 0) {
        throw new Error("WASM JIT request contains a malformed module payload.");
      }
      return total + bytes;
    }, 0);
    if (
      preparation.artifacts.length > WASM_JIT_CACHE_MAX_MODELS ||
      requiredBytes > WASM_JIT_CACHE_MAX_BYTES
    ) {
      const error = `WASM JIT request requires ${preparation.artifacts.length} models and ${requiredBytes} bytes, exceeding the bounded worker cache.`;
      wasmJitCapability = { ...wasmJitCapability, lastRequestError: error };
      console.warn(error);
      return candidateToken;
    }
    for (const error of preparation.errors) {
      console.warn(String(error));
    }
  } catch (error) {
    if (Number.isInteger(candidateToken) && candidateToken > 0) {
      module.cancelPreparedRspiceUiWasmJitRequest(candidateToken);
    }
    throw error;
  }
  try {
    for (const artifact of preparation.artifacts) {
      await installWasmJitArtifact(module, artifact);
    }
  } catch (error) {
    const message = asErrorMessage(error);
    wasmJitCapability = { ...wasmJitCapability, lastRequestError: message };
    console.warn(`RSpice WASM JIT request installation failed: ${message}`);
  }
  return candidateToken;
}

function dispatchWasmJitEntry(cacheKey, exportName, frameOffset) {
  if (!WASM_JIT_IDENTITY.test(cacheKey || "")) {
    throw new Error("WASM JIT dispatch received an invalid model cache key.");
  }
  if (typeof exportName !== "string" || exportName.length === 0) {
    throw new Error("WASM JIT dispatch received an invalid export name.");
  }
  if (!Number.isInteger(frameOffset) || frameOffset < 0 || frameOffset > 0xffff_ffff) {
    throw new Error("WASM JIT dispatch received an invalid frame offset.");
  }
  const installed = wasmJitModelCache.get(cacheKey);
  if (!installed) {
    throw new Error(`WASM JIT model ${cacheKey} is not installed.`);
  }
  const entry = installed.instance.exports[exportName];
  if (typeof entry !== "function") {
    throw new Error(`WASM JIT model ${cacheKey} has no export ${exportName}.`);
  }
  const status = entry(frameOffset);
  if (!Number.isInteger(status) || status < -0x8000_0000 || status > 0x7fff_ffff) {
    throw new Error(`WASM JIT export ${exportName} returned an invalid status.`);
  }
  return status;
}

async function initializeWorkerModule() {
  const module = await import(executableAsset("rspice-ui-worker.js").href);
  const wasmModule = executableAsset("rspice-ui-worker_bg.wasm");
  const wasmExports = await module.default({ module_or_path: wasmModule });
  primaryWasmExports = wasmExports;
  workerModule = module;
  if (typeof module.runRspiceUiWorkerRequest !== "function") {
    throw new Error("RSpice worker package is missing its request executor.");
  }
  if (typeof module.prepareRspiceUiWasmJitRequest !== "function") {
    throw new Error("RSpice worker package is missing its WASM JIT request preparer.");
  }
  if (typeof module.runPreparedRspiceUiWasmJitRequest !== "function") {
    throw new Error("RSpice worker package is missing its prepared request executor.");
  }
  if (typeof module.cancelPreparedRspiceUiWasmJitRequest !== "function") {
    throw new Error("RSpice worker package is missing its prepared request canceller.");
  }
  if (typeof module.installRspiceUiWasmJitDispatcher !== "function") {
    throw new Error("RSpice worker package is missing its WASM JIT dispatcher installer.");
  }
  if (typeof module.runRspiceUiVerilogACompileRequest !== "function") {
    throw new Error("RSpice worker package is missing its Verilog-A compiler executor.");
  }
  if (typeof module.runRspiceUiHardcopyRequest !== "function") {
    throw new Error("RSpice worker package is missing its hardcopy executor.");
  }
  if (typeof module.runRspiceUiModelImportRequest !== "function") {
    throw new Error("RSpice worker package is missing its model-import executor.");
  }
  if (typeof module.runRspiceUiPdkImportRequest !== "function") {
    throw new Error("RSpice worker package is missing its PDK-import executor.");
  }
  runWorkerRequest = module.runRspiceUiWorkerRequest;
  runVerilogACompileRequest = module.runRspiceUiVerilogACompileRequest;
  runHardcopyRequest = module.runRspiceUiHardcopyRequest;
  runModelImportRequest = module.runRspiceUiModelImportRequest;
  runPdkImportRequest = module.runRspiceUiPdkImportRequest;
  module.installRspiceUiWasmJitDispatcher(dispatchWasmJitEntry);
  wasmJitCapability = await qualifyWasmJitArchitecture(module, wasmExports);
}

async function ensureReady() {
  if (!initPromise) {
    initPromise = initializeWorkerModule().catch((error) => {
      initPromise = null;
      runWorkerRequest = null;
      runVerilogACompileRequest = null;
      runHardcopyRequest = null;
      runModelImportRequest = null;
      runPdkImportRequest = null;
      primaryWasmExports = null;
      workerModule = null;
      wasmJitModelCache.clear();
      wasmJitCacheBytes = 0;
      wasmJitCapability = {
        available: false,
        reason: asErrorMessage(error),
      };
      throw error;
    });
  }
  await initPromise;
}

self.addEventListener("message", (event) => {
  const message = event.data || {};
  if (message.type === "run-pdk-import") {
    void (async () => {
      try {
        await ensureReady();
        const response = runPdkImportRequest(message.request);
        postMessage(
          { type: "pdk-import-result", id: message.id, response },
          pdkImportResponseTransferList(response),
        );
      } catch (error) {
        postMessage({
          type: "pdk-import-error",
          id: message.id ?? 0,
          error: asErrorMessage(error),
        });
      }
    })();
    return;
  }
  if (message.type === "run-model-import") {
    void (async () => {
      try {
        await ensureReady();
        const response = runModelImportRequest(message.request);
        postMessage(
          { type: "model-import-result", id: message.id, response },
          modelImportResponseTransferList(response),
        );
      } catch (error) {
        postMessage({
          type: "model-import-error",
          id: message.id ?? 0,
          error: asErrorMessage(error),
        });
      }
    })();
    return;
  }
  if (message.type === "run-hardcopy") {
    void (async () => {
      try {
        await ensureReady();
        const request = message.request;
        if (
          !request ||
          !request.metadata ||
          request.metadata.protocolVersion !== HARDCOPY_PROTOCOL_VERSION ||
          !Array.isArray(request.buffers)
        ) {
          throw new Error("Unsupported or malformed RSpice hardcopy request transport.");
        }
        const response = runHardcopyRequest(request);
        postMessage(
          { type: "hardcopy-result", id: message.id, response },
          hardcopyResponseTransferList(response),
        );
      } catch (error) {
        postMessage({
          type: "hardcopy-error",
          id: message.id ?? 0,
          error: asErrorMessage(error),
        });
      }
    })();
    return;
  }
  if (message.type === "compile-veriloga") {
    void (async () => {
      try {
        await ensureReady();
        const response = runVerilogACompileRequest(message.request);
        await installWasmJitModel(workerModule, response);
        delete response.wasmJitArtifact;
        delete response.wasmJitError;
        postMessage({ type: "veriloga-result", id: message.id, response });
      } catch (error) {
        postMessage({
          type: "veriloga-error",
          id: message.id ?? 0,
          error: asErrorMessage(error),
        });
      }
    })();
    return;
  }
  if (message.type !== "run") {
    return;
  }

  void (async () => {
    try {
      await ensureReady();
      const request = message.request;
      if (
        !request ||
        request.protocolVersion !== WORKER_REQUEST_PROTOCOL_VERSION ||
        !Array.isArray(request.buffers)
      ) {
        throw new Error("Unsupported or malformed RSpice worker request transport.");
      }
      let dispatchToken;
      try {
        dispatchToken = await prepareWasmJitSimulationRequest(workerModule, request);
      } catch (error) {
        wasmJitCapability = {
          ...wasmJitCapability,
          lastRequestError: asErrorMessage(error),
        };
        throw error;
      }
      const response = workerModule.runPreparedRspiceUiWasmJitRequest(dispatchToken);
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
    postMessage({ type: "ready", wasmJit: wasmJitCapability });
  })
  .catch((error) => {
    postMessage({ type: "error", id: 0, error: asErrorMessage(error) });
  });
