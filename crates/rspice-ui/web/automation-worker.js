const PROTOCOL = Object.freeze({ major: 1, minor: 4 });
const PYODIDE_VERSION = "314.0.2";
const PYTHON_VERSION = "3.14.2";
const RSPICE_API_VERSION = "1.0.0";
const RUNTIME_BUILD = `pyodide-${PYODIDE_VERSION}`;
const WASM_PAGE_BYTES = 64 * 1024;
const BROWSER_MEMORY_LIMIT_BYTES = 2 * 1024 * 1024 * 1024;
const UPSTREAM_PYODIDE_MAXIMUM_MEMORY_PAGES = 65_536;
const RSPICE_PYODIDE_MAXIMUM_MEMORY_PAGES =
  BROWSER_MEMORY_LIMIT_BYTES / WASM_PAGE_BYTES;
const MAX_BROWSER_WALL_TIME_MS = 24 * 60 * 60 * 1000;
const MAX_BROWSER_OUTPUT_BYTES = 16 * 1024 * 1024;
const MAX_BROWSER_ARTIFACT_BYTES = 512 * 1024 * 1024;
const MAX_BROWSER_STACK_DEPTH = 4_000;
const RUNTIME_DIGEST_HEX =
  "f7eb926d7da72f7a2a62b2e17dfb6dc8bca436446cfa0329c895c45deaa2a360";
const BASE_ENVIRONMENT_DIGEST_HEX =
  "d445b1443965be4e6b1b191ee023176dbd35430ac3cd00603458384ea03b8518";
const BOOTSTRAP = Object.freeze([
  "rspice_browser_bootstrap.py",
  39702,
  "6133114210f2620385ec3eb91c8e30876f7af8c5ddde1151a0aa6286d9aa4831",
]);
const PYODIDE_FILES = Object.freeze([
  ["pyodide.mjs", 17880, "955d2088bbb7fc79a73c4802aca2370c1d95bfdfaffa4121e0faebda2b0ea3f9"],
  ["pyodide.asm.mjs", 1250259, "c7eccdfeb7a8419d61f910f0685b45cd5610b7ff5bbe844c3c1050ee6623b641"],
  ["pyodide.asm.wasm", 9609998, "f7a8a169e513791e18fa0790fb69d6f2656b779e9012ba57e03e973f0df0b39f"],
  ["python_stdlib.zip", 2552456, "101a9c94ca6304c1478c89b7b595136b9a51b4289bdc5b467d86db553efee9b3"],
  ["pyodide-lock.json", 113804, "c963d22858f6bcb8f41586a2142f03905ab370c88ea22a86a2736e95fac2a8f3"],
]);

let sequence = 0;
let pyodidePromise = null;
let active = null;
let nextCallId = 1;
const pendingCalls = new Map();
const debugCommands = [];
let debugCommandResolver = null;

function postEnvelope(event, requestId = null, sessionId = null) {
  sequence += 1;
  postMessage({
    type: "automation-event",
    envelope: {
      protocol: PROTOCOL,
      request_id: requestId,
      session_id: sessionId,
      sequence,
      event,
    },
  });
}

function runtimeIdentity() {
  return {
    managed: true,
    platform: "browser-wasm",
    architecture: "wasm32",
    runtime_build: RUNTIME_BUILD,
    runtime_digest: Array.from(
      RUNTIME_DIGEST_HEX.match(/../g),
      (pair) => Number.parseInt(pair, 16),
    ),
    python_version: PYTHON_VERSION,
    python_abi: "cp314-emscripten_5_0_3-wasm32",
    rspice_api_version: RSPICE_API_VERSION,
    protocol: PROTOCOL,
  };
}

function hex(bytes) {
  return Array.from(new Uint8Array(bytes), (value) =>
    value.toString(16).padStart(2, "0"),
  ).join("");
}

async function fetchVerified(url, expectedBytes, expectedSha256) {
  const response = await fetch(url, {
    cache: "force-cache",
    credentials: "omit",
    redirect: "error",
  });
  if (!response.ok) {
    throw new Error(`Could not load pinned browser runtime asset ${url.pathname}.`);
  }
  const body = await response.arrayBuffer();
  if (body.byteLength !== expectedBytes) {
    throw new Error(`Pinned browser runtime asset ${url.pathname} has the wrong size.`);
  }
  const digest = hex(await crypto.subtle.digest("SHA-256", body));
  if (digest !== expectedSha256) {
    throw new Error(`Pinned browser runtime asset ${url.pathname} failed SHA-256 verification.`);
  }
  return body;
}

function readUnsignedLeb128(bytes, start) {
  let value = 0;
  let shift = 0;
  let offset = start;
  while (offset < bytes.length && shift <= 49) {
    const byte = bytes[offset];
    value += (byte & 0x7f) * (2 ** shift);
    offset += 1;
    if ((byte & 0x80) === 0) {
      if (!Number.isSafeInteger(value)) {
        throw new Error("Pinned Pyodide WebAssembly contains an unsafe LEB128 value.");
      }
      return { value, offset };
    }
    shift += 7;
  }
  throw new Error("Pinned Pyodide WebAssembly contains an invalid LEB128 value.");
}

function encodeUnsignedLeb128(value) {
  if (!Number.isSafeInteger(value) || value < 0) {
    throw new Error("RSpice WebAssembly memory policy is invalid.");
  }
  const encoded = [];
  do {
    let byte = value & 0x7f;
    value = Math.floor(value / 128);
    if (value !== 0) {
      byte |= 0x80;
    }
    encoded.push(byte);
  } while (value !== 0);
  return encoded;
}

function applyPyodideMemoryLimit(source) {
  const bytes = new Uint8Array(source.slice(0));
  const header = [0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
  if (header.some((value, index) => bytes[index] !== value)) {
    throw new Error("Pinned Pyodide WebAssembly has an invalid module header.");
  }
  let offset = header.length;
  let memorySections = 0;
  while (offset < bytes.length) {
    const sectionId = bytes[offset];
    offset += 1;
    const sectionSize = readUnsignedLeb128(bytes, offset);
    offset = sectionSize.offset;
    const sectionEnd = offset + sectionSize.value;
    if (sectionEnd > bytes.length) {
      throw new Error("Pinned Pyodide WebAssembly has a truncated section.");
    }
    if (sectionId === 5) {
      memorySections += 1;
      const count = readUnsignedLeb128(bytes, offset);
      offset = count.offset;
      if (count.value !== 1) {
        throw new Error("Pinned Pyodide WebAssembly must define exactly one memory.");
      }
      const flags = readUnsignedLeb128(bytes, offset);
      offset = flags.offset;
      if (flags.value !== 1) {
        throw new Error("Pinned Pyodide WebAssembly memory must have one fixed maximum.");
      }
      const minimum = readUnsignedLeb128(bytes, offset);
      offset = minimum.offset;
      if (minimum.value > RSPICE_PYODIDE_MAXIMUM_MEMORY_PAGES) {
        throw new Error("Pinned Pyodide WebAssembly minimum exceeds the RSpice memory policy.");
      }
      const maximumStart = offset;
      const maximum = readUnsignedLeb128(bytes, offset);
      offset = maximum.offset;
      if (maximum.value !== UPSTREAM_PYODIDE_MAXIMUM_MEMORY_PAGES) {
        throw new Error("Pinned Pyodide WebAssembly maximum no longer matches its qualified upstream build.");
      }
      const replacement = encodeUnsignedLeb128(RSPICE_PYODIDE_MAXIMUM_MEMORY_PAGES);
      if (replacement.length !== maximum.offset - maximumStart || offset !== sectionEnd) {
        throw new Error("Pinned Pyodide WebAssembly memory layout is not safely patchable.");
      }
      bytes.set(replacement, maximumStart);
      offset = sectionEnd;
      continue;
    }
    offset = sectionEnd;
  }
  if (memorySections !== 1) {
    throw new Error("Pinned Pyodide WebAssembly has no unique defined memory.");
  }
  return bytes;
}

function runtimeRoot() {
  return new URL("./python/pyodide-314.0.2/", import.meta.url);
}

async function initializePyodide() {
  const root = runtimeRoot();
  const verifiedAssets = await Promise.all(
    PYODIDE_FILES.map(([path, bytes, digest]) =>
      fetchVerified(new URL(path, root), bytes, digest),
    ),
  );
  const wasmBinary = applyPyodideMemoryLimit(verifiedAssets[2]);
  const bootstrapBody = await fetchVerified(
    new URL(`./python/${BOOTSTRAP[0]}`, import.meta.url),
    BOOTSTRAP[1],
    BOOTSTRAP[2],
  );
  const bootstrap = new TextDecoder("utf-8", { fatal: true }).decode(bootstrapBody);
  const [{ loadPyodide }, { default: createUpstreamPyodideModule }] = await Promise.all([
    import(new URL("pyodide.mjs", root).href),
    import(new URL("pyodide.asm.mjs", root).href),
  ]);
  const runtime = await loadPyodide({
    indexURL: root.href,
    lockFileURL: new URL("pyodide-lock.json", root).href,
    enableRunUntilComplete: true,
    checkAPIVersion: true,
    createPyodideModule: (settings) =>
      createUpstreamPyodideModule({ ...settings, wasmBinary }),
    env: {
      RSPICE_API_VERSION,
      PYTHONNOUSERSITE: "1",
      PYTHONDONTWRITEBYTECODE: "1",
    },
  });
  const actualPythonVersion = String(
    runtime.runPython(
      "import sys; '.'.join(str(part) for part in sys.version_info[:3])",
      { filename: "rspice-runtime://identity.py" },
    ),
  );
  if (actualPythonVersion !== PYTHON_VERSION) {
    throw new Error(
      `Pinned Pyodide Python identity mismatch: expected ${PYTHON_VERSION}, received ${actualPythonVersion}.`,
    );
  }
  runtime.registerJsModule("rspice_bridge", {
    host_call: (payload) => browserHostCall(String(payload)),
    debug_exchange: (payload) => browserDebugExchange(String(payload)),
    debug_checkpoint: () => browserDebugCheckpoint(),
  });
  // Keep the trusted worker implementation in a private namespace for the
  // worker lifetime. Launch inputs are supplied through short-lived locals
  // below, while project code is still executed in its own isolated globals.
  // This prevents both accidental symbol loss between Pyodide evaluations and
  // exposure of the host bridge to user Python.
  const trustedGlobals = runtime.toPy({
    __name__: "_rspice_runtime_internal",
    __file__: "rspice-runtime://rspice_browser_bootstrap.py",
  });
  await runtime.runPythonAsync(bootstrap, {
    globals: trustedGlobals,
    filename: "rspice-runtime://rspice_browser_bootstrap.py",
  });
  // Remove ambient Web APIs after the trusted runtime has fetched and
  // initialized its immutable assets. User Python retains only host_call.
  for (const name of [
    "fetch",
    "XMLHttpRequest",
    "WebSocket",
    "EventSource",
    "indexedDB",
    "caches",
    "importScripts",
  ]) {
    try {
      Reflect.deleteProperty(globalThis, name);
    } catch (_) {
      // Some browser globals are non-configurable. Python's audit guard still
      // blocks direct js/pyodide imports, and the worker has no DOM authority.
    }
  }
  return { runtime, trustedGlobals };
}

function ensurePyodide() {
  if (!pyodidePromise) {
    pyodidePromise = initializePyodide().catch((error) => {
      pyodidePromise = null;
      throw error;
    });
  }
  return pyodidePromise;
}

function newSessionId() {
  return crypto.randomUUID();
}

function browserHostCall(payload) {
  if (!active) {
    return Promise.reject(new Error("No authenticated RSpice browser session is active."));
  }
  let request;
  try {
    request = JSON.parse(payload);
  } catch (_) {
    return Promise.reject(new Error("RSpice host-call payload is not valid JSON."));
  }
  const callId = nextCallId++;
  return new Promise((resolve, reject) => {
    pendingCalls.set(callId, { resolve, reject });
    postEnvelope(
      {
        event: "host-call",
        call: {
          call_id: callId,
          capability: request.capability,
          capability_token: request.capability_token,
          operation: request.operation,
        },
      },
      active.requestId,
      active.sessionId,
    );
  });
}

function postDebugEvents(payload) {
  if (!active || !payload || !Array.isArray(payload.events)) {
    throw new Error("Browser debugger emitted an invalid event batch.");
  }
  for (const item of payload.events) {
    if (!item || typeof item !== "object" || !item.event) {
      throw new Error("Browser debugger event batch contains an invalid item.");
    }
    postEnvelope(
      item.event,
      Number.isSafeInteger(item.request_id) ? item.request_id : active.requestId,
      active.sessionId,
    );
  }
}

function nextDebugCommand() {
  return debugCommands.shift() || { operation: "none" };
}

function enqueueDebugCommand(command) {
  if (!active || active.mode !== "debug") {
    throw new Error("No authenticated browser debugger session is active.");
  }
  if (debugCommandResolver) {
    const resolve = debugCommandResolver;
    debugCommandResolver = null;
    resolve(JSON.stringify(command));
  } else {
    debugCommands.push(command);
  }
}

function browserDebugExchange(payloadText) {
  const payload = JSON.parse(String(payloadText));
  postDebugEvents(payload);
  if (payload.wait === false) {
    return Promise.resolve(JSON.stringify({ operation: "none" }));
  }
  const queued = nextDebugCommand();
  if (queued.operation !== "none") {
    return Promise.resolve(JSON.stringify(queued));
  }
  if (debugCommandResolver) {
    return Promise.reject(new Error("Browser debugger already has a pending command exchange."));
  }
  return new Promise((resolve) => {
    debugCommandResolver = resolve;
  });
}

function browserDebugCheckpoint() {
  return new Promise((resolve) => {
    setTimeout(() => resolve(JSON.stringify(nextDebugCommand())), 0);
  });
}

function parseVersion(value) {
  const match = /^(\d+)\.(\d+)\.(\d+)$/.exec(value);
  return match ? match.slice(1).map(Number) : null;
}

function compareVersion(left, right) {
  for (let index = 0; index < 3; index += 1) {
    if (left[index] !== right[index]) {
      return left[index] < right[index] ? -1 : 1;
    }
  }
  return 0;
}

function matchesRequirement(versionText, requirement) {
  const version = parseVersion(versionText);
  if (!version || typeof requirement !== "string" || !requirement.trim()) {
    return false;
  }
  return requirement.split(",").every((raw) => {
    const clause = raw.trim();
    const match = /^(>=|<=|>|<|=|~|\^)?\s*(\d+)(?:\.(\d+))?(?:\.(\d+))?$/.exec(clause);
    if (!match) {
      return false;
    }
    const target = [Number(match[2]), Number(match[3] || 0), Number(match[4] || 0)];
    const comparison = compareVersion(version, target);
    switch (match[1] || "^") {
      case "=": return comparison === 0;
      case ">=": return comparison >= 0;
      case "<=": return comparison <= 0;
      case ">": return comparison > 0;
      case "<": return comparison < 0;
      case "~": return comparison >= 0 && version[0] === target[0] && version[1] === target[1];
      case "^": return comparison >= 0 && version[0] === target[0];
      default: return false;
    }
  });
}

function validateLaunch(request) {
  const snapshot = request.snapshot;
  const limits = request.limits;
  if (!snapshot || typeof snapshot !== "object" || !Array.isArray(snapshot.documents)) {
    throw new Error("Browser Automation launch has no immutable source snapshot.");
  }
  if (!matchesRequirement(PYTHON_VERSION, snapshot.python_requirement)) {
    throw new Error(
      `Pinned browser Python ${PYTHON_VERSION} does not satisfy ${snapshot.python_requirement}.`,
    );
  }
  if (!matchesRequirement(RSPICE_API_VERSION, snapshot.api_requirement)) {
    throw new Error(
      `RSpice API ${RSPICE_API_VERSION} does not satisfy ${snapshot.api_requirement}.`,
    );
  }
  if (!matchesRequirement(PYODIDE_VERSION, snapshot.browser_runtime_requirement)) {
    throw new Error(
      `Pinned browser runtime ${PYODIDE_VERSION} does not satisfy ${snapshot.browser_runtime_requirement}.`,
    );
  }
  const environmentDigest = Array.isArray(snapshot.environment_digest)
    ? snapshot.environment_digest.map((value) => Number(value).toString(16).padStart(2, "0")).join("")
    : "";
  if (environmentDigest !== BASE_ENVIRONMENT_DIGEST_HEX) {
    throw new Error(
      "The selected Python environment is not installed in this immutable browser release.",
    );
  }
  if (!limits || typeof limits !== "object") {
    throw new Error("Browser Automation resource limits are missing.");
  }
  for (const field of [
    "wall_time_ms",
    "cpu_time_ms",
    "memory_bytes",
    "output_bytes",
    "artifact_bytes",
    "max_tasks",
    "max_stack_depth",
  ]) {
    if (!Number.isSafeInteger(limits[field]) || limits[field] <= 0) {
      throw new Error(`Browser Automation resource limit ${field} is invalid.`);
    }
  }
  if (limits.memory_bytes !== BROWSER_MEMORY_LIMIT_BYTES) {
    throw new Error("Browser Automation must use the hard 2 GiB Pyodide memory limit.");
  }
  if (limits.cpu_time_ms !== limits.wall_time_ms) {
    throw new Error("Browser Automation CPU and wall limits must match in its single worker.");
  }
  if (limits.max_tasks !== 1) {
    throw new Error("Browser Automation permits exactly one isolated task.");
  }
  for (const [field, maximum] of [
    ["wall_time_ms", MAX_BROWSER_WALL_TIME_MS],
    ["output_bytes", MAX_BROWSER_OUTPUT_BYTES],
    ["artifact_bytes", MAX_BROWSER_ARTIFACT_BYTES],
    ["max_stack_depth", MAX_BROWSER_STACK_DEPTH],
  ]) {
    if (limits[field] > maximum) {
      throw new Error(`Browser Automation resource limit ${field} exceeds product policy.`);
    }
  }
  return { snapshot, limits };
}

async function launch(envelope) {
  if (active) {
    throw new Error("A browser Automation session is already active.");
  }
  const requestId = envelope.request_id;
  const request = envelope.request;
  const sessionId = newSessionId();
  const { snapshot, limits } = validateLaunch(request);
  active = { requestId, sessionId, mode: request.mode };
  nextCallId = 1;
  debugCommands.length = 0;
  debugCommandResolver = null;
  postEnvelope(
    { event: "state", state: "validating", detail: "verifying pinned Pyodide and compiling the exact source closure" },
    requestId,
    sessionId,
  );
  const { runtime, trustedGlobals } = await ensurePyodide();
  const locals = runtime.toPy({ snapshotJson: JSON.stringify(snapshot) });
  let validation;
  try {
    const result = await runtime.runPythonAsync(
      "validate_browser_snapshot(snapshotJson)",
      {
        globals: trustedGlobals,
        locals,
        filename: "rspice-runtime://validate.py",
      },
    );
    validation = JSON.parse(String(result));
    result.destroy?.();
  } finally {
    locals.destroy();
  }
  for (const diagnostic of validation) {
    postEnvelope({ event: "diagnostic", diagnostic }, requestId, sessionId);
  }
  if (validation.length) {
    postEnvelope(
      { event: "state", state: "failed", detail: "managed Pyodide rejected one or more source documents" },
      requestId,
      sessionId,
    );
    active = null;
    return;
  }
  if (request.mode === "validate") {
    postEnvelope(
      { event: "state", state: "completed", detail: "managed Pyodide accepted the exact source closure" },
      requestId,
      sessionId,
    );
    active = null;
    return;
  }
  if (request.mode !== "run" && request.mode !== "dry-run" && request.mode !== "debug") {
    throw new Error(`Unsupported browser Automation launch mode ${request.mode}.`);
  }
  postEnvelope(
    { event: "state", state: "running", detail: "executing governed Python through the RSpice capability broker" },
    requestId,
    sessionId,
  );
  const executionLocals = runtime.toPy({
    snapshotJson: JSON.stringify(snapshot),
    mode: request.mode,
    outputLimit: limits.output_bytes,
    maxStackDepth: limits.max_stack_depth,
    breakpointsJson: JSON.stringify(request.breakpoints || []),
    exceptionPolicy: request.exception_policy,
  });
  let execution;
  const captured = [];
  try {
    do {
      const result = await runtime.runPythonAsync(
        "execute_browser_snapshot(snapshotJson, mode, outputLimit, maxStackDepth, breakpointsJson, exceptionPolicy)",
        {
          globals: trustedGlobals,
          locals: executionLocals,
          filename: "rspice-runtime://execute.py",
        },
      );
      execution = JSON.parse(String(result));
      result.destroy?.();
      captured.push([execution.stdout, execution.stderr]);
    } while (execution.restart);
  } finally {
    executionLocals.destroy();
  }
  for (const output of captured) {
    for (const [channel, text] of [["stdout", output[0]], ["stderr", output[1]]]) {
      if (text) {
        postEnvelope(
          { event: "output", channel, category: "python", text },
          requestId,
          sessionId,
        );
      }
    }
  }
  if (execution.cancelled) {
    postEnvelope(
      { event: "state", state: "cancelled", detail: "browser Automation session was cancelled" },
      requestId,
      sessionId,
    );
  } else if (!execution.ok) {
    if (execution.traceback) {
      postEnvelope(
        { event: "output", channel: "stderr", category: "traceback", text: execution.traceback },
        requestId,
        sessionId,
      );
    }
    postEnvelope(
      { event: "worker-failed", code: "PYTHON-EXCEPTION", message: execution.error || "Python execution failed", recoverable: true },
      requestId,
      sessionId,
    );
    postEnvelope(
      { event: "state", state: "failed", detail: "Python execution raised an exception" },
      requestId,
      sessionId,
    );
  } else {
    postEnvelope(
      { event: "state", state: "completed", detail: "governed browser Python execution completed" },
      requestId,
      sessionId,
    );
  }
  debugCommands.length = 0;
  debugCommandResolver = null;
  active = null;
}

function handleHostResponse(request) {
  if (!active || request.session_id !== active.sessionId) {
    throw new Error("Host response has no matching browser Automation session.");
  }
  const pending = pendingCalls.get(request.call_id);
  if (!pending) {
    throw new Error("Host response has no matching pending capability call.");
  }
  pendingCalls.delete(request.call_id);
  pending.resolve(JSON.stringify(request.response));
}

function cancel(request) {
  if (!active || request.session_id !== active.sessionId) {
    return;
  }
  if (active.mode === "debug") {
    for (const pending of pendingCalls.values()) {
      pending.resolve(JSON.stringify({
        status: "failure",
        code: "CANCELLED",
        message: "Automation session was cancelled.",
        permission_denied: false,
      }));
    }
    pendingCalls.clear();
    enqueueDebugCommand({ operation: "stop" });
    return;
  }
  for (const pending of pendingCalls.values()) {
    pending.reject(new Error("Automation session was cancelled."));
  }
  pendingCalls.clear();
  postEnvelope(
    { event: "state", state: "cancelled", detail: "browser Automation session was cancelled" },
    active.requestId,
    active.sessionId,
  );
  active = null;
}

self.addEventListener("message", (event) => {
  const message = event.data || {};
  if (message.type !== "automation-request") {
    return;
  }
  const envelope = message.envelope;
  void (async () => {
    try {
      if (
        !envelope ||
        envelope.protocol?.major !== PROTOCOL.major ||
        envelope.protocol?.minor > PROTOCOL.minor ||
        !Number.isSafeInteger(envelope.request_id) ||
        envelope.request_id <= 0
      ) {
        throw new Error("Malformed or incompatible browser Automation request envelope.");
      }
      switch (envelope.request?.operation) {
        case "probe":
          postEnvelope({ event: "hello", identity: runtimeIdentity() }, envelope.request_id);
          break;
        case "launch":
          await launch(envelope);
          break;
        case "host-response":
          handleHostResponse(envelope.request);
          break;
        case "debug-control":
          if (!active || envelope.request.session_id !== active.sessionId) {
            throw new Error("Debugger request has no matching browser session.");
          }
          enqueueDebugCommand({
            ...envelope.request,
            operation: envelope.request.control,
            request_id: envelope.request_id,
          });
          break;
        case "set-breakpoints":
        case "stack-trace":
        case "variables":
        case "evaluate":
          if (!active || envelope.request.session_id !== active.sessionId) {
            throw new Error("Debugger request has no matching browser session.");
          }
          enqueueDebugCommand({
            ...envelope.request,
            request_id: envelope.request_id,
          });
          break;
        case "cancel":
          cancel(envelope.request);
          break;
        case "shutdown":
          postEnvelope({ event: "terminated", exit_code: 0, reason: "browser worker shutdown requested" }, envelope.request_id);
          self.close();
          break;
        default:
          throw new Error(`Unsupported browser Automation request ${envelope.request?.operation}.`);
      }
    } catch (error) {
      const requestId = envelope?.request_id ?? null;
      const sessionId = active?.sessionId ?? null;
      postEnvelope(
        { event: "worker-failed", code: "BROWSER-WORKER-FAILED", message: error instanceof Error ? error.message : String(error), recoverable: false },
        requestId,
        sessionId,
      );
      if (sessionId) {
        postEnvelope(
          { event: "state", state: "failed", detail: "browser Python worker failed closed" },
          requestId,
          sessionId,
        );
      }
      if (debugCommandResolver) {
        const resolve = debugCommandResolver;
        debugCommandResolver = null;
        resolve(JSON.stringify({ operation: "stop" }));
      }
      debugCommands.length = 0;
      active = null;
    }
  })();
});

ensurePyodide()
  .then(() => postEnvelope({ event: "hello", identity: runtimeIdentity() }))
  .catch((error) => {
    postEnvelope({
      event: "worker-failed",
      code: "BROWSER-RUNTIME-INTEGRITY",
      message: error instanceof Error ? error.message : String(error),
      recoverable: false,
    });
  });
