// The frames a worker host trades with the mixed corpus runner, and the
// conversation it holds with the application's `simulation-worker.js`.
// Plain JavaScript over `Uint8Array`, so the Node host (`worker_host.mjs`)
// and the browser host (`tools/ci/browser_worker_host.py`) share it.
//
// A frame is a u32 header length, the header as JSON, a u32 blob count, then
// each blob as a u32 length and its bytes, every integer little-endian.
//
// To the worker: {"id", "protocol", "floats", "bytes"} with blobs [request
// metadata JSON, ...float buffers (f64), ...byte buffers].
// From it: {"type": "ready"} once the worker is up; {"type": "checkpoint",
// "id"} with the transient state the run streamed; {"type": "log", "id",
// "message"} per engine log line; {"type": "result", "id", "protocol",
// "floats"} with blobs [response metadata JSON, ...float buffers]; or
// {"type": "error", "id", "error"}.

const encoder = new TextEncoder();
const decoder = new TextDecoder();

export function encodeFrame(header, blobs = []) {
  const text = encoder.encode(JSON.stringify(header));
  const views = blobs.map((blob) =>
    blob instanceof Uint8Array ? blob : new Uint8Array(blob.buffer, blob.byteOffset, blob.byteLength),
  );
  const size = 8 + text.length + views.reduce((total, view) => total + 4 + view.length, 0);
  const bytes = new Uint8Array(size);
  const data = new DataView(bytes.buffer);
  let offset = 0;
  const u32 = (value) => {
    data.setUint32(offset, value, true);
    offset += 4;
  };
  u32(text.length);
  bytes.set(text, offset);
  offset += text.length;
  u32(views.length);
  for (const view of views) {
    u32(view.length);
    bytes.set(view, offset);
    offset += view.length;
  }
  return bytes;
}

// The first whole frame in `bytes`, or null while it is still arriving.
export function decodeFrame(bytes) {
  const data = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  let offset = 0;
  const u32 = () => {
    if (offset + 4 > bytes.length) {
      return null;
    }
    const value = data.getUint32(offset, true);
    offset += 4;
    return value;
  };
  const take = (length) => {
    if (length === null || offset + length > bytes.length) {
      return null;
    }
    const slice = bytes.slice(offset, offset + length);
    offset += length;
    return slice;
  };
  const header = take(u32());
  const count = header && u32();
  if (header === null || count === null) {
    return null;
  }
  const blobs = [];
  for (let index = 0; index < count; index += 1) {
    const blob = take(u32());
    if (blob === null) {
      return null;
    }
    blobs.push(blob);
  }
  return { header: JSON.parse(decoder.decode(header)), blobs, size: offset };
}

// The `run` message the application's main thread posts for one request
// frame, and the buffers it transfers.
export function runMessage(header, blobs) {
  const metadata = JSON.parse(decoder.decode(blobs[0]));
  const buffers = blobs.slice(1, 1 + header.floats).map((blob) => {
    const values = new Float64Array(blob.length / 8);
    new Uint8Array(values.buffer).set(blob);
    return values;
  });
  const byteBuffers = blobs
    .slice(1 + header.floats, 1 + header.floats + header.bytes)
    .map((blob) => new Uint8Array(blob));
  const request = { protocolVersion: header.protocol, request: metadata, buffers, byteBuffers };
  const transfer = [...buffers, ...byteBuffers].map((view) => view.buffer);
  return { message: { type: "run", id: header.id, request }, transfer };
}

// The frame for a message the worker posted, or null for one the corpus has
// no use for (progress, samples, notices).
export function eventFrame(message) {
  switch (message?.type) {
    case "ready":
      return encodeFrame({ type: "ready" });
    case "transientCheckpoint":
      return encodeFrame({ type: "checkpoint", id: message.id }, [message.checkpoint]);
    case "engineLog":
      return typeof message.line?.message === "string"
        ? encodeFrame({ type: "log", id: message.id, message: message.line.message })
        : null;
    case "result":
      return resultFrame(message);
    case "error":
      return encodeFrame({ type: "error", id: message.id ?? 0, error: String(message.error) });
    default:
      return null;
  }
}

// The worker's answer: its metadata as JSON and its buffers as bytes.
function resultFrame(message) {
  const response = message.response;
  let metadata;
  try {
    metadata = toJson(response.response, "response");
  } catch (error) {
    return encodeFrame({
      type: "error",
      id: message.id,
      error: `the worker's answer: ${error.message}`,
    });
  }
  return encodeFrame(
    {
      type: "result",
      id: message.id,
      protocol: response.protocolVersion,
      floats: response.buffers.length,
    },
    [encoder.encode(metadata), ...response.buffers],
  );
}

// JSON for a value structured clone delivered: a Map as an object, a BigInt
// by its digits, -0 kept negative, and a number JSON cannot spell refused by
// where it sits rather than written as something else.
export function toJson(value, where) {
  if (value === null || value === undefined) {
    return "null";
  }
  switch (typeof value) {
    case "boolean":
      return value ? "true" : "false";
    case "bigint":
      return value.toString();
    case "string":
      return JSON.stringify(value);
    case "number":
      if (!Number.isFinite(value)) {
        throw new Error(`${where} is ${value}, which JSON cannot carry`);
      }
      return Object.is(value, -0) ? "-0.0" : JSON.stringify(value);
    default:
      break;
  }
  if (ArrayBuffer.isView(value) || Array.isArray(value)) {
    const items = Array.from(value, (item, index) => toJson(item, `${where}[${index}]`));
    return `[${items.join(",")}]`;
  }
  const entries = value instanceof Map ? [...value.entries()] : Object.entries(value);
  const fields = [];
  for (const [key, item] of entries) {
    if (item !== undefined) {
      fields.push(`${JSON.stringify(String(key))}:${toJson(item, `${where}.${key}`)}`);
    }
  }
  return `{${fields.join(",")}}`;
}
