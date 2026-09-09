// Independent ABI 12 release fixture. Keep these bytes and opcodes explicit:
// deriving them from the compiler would let both sides drift together.
const ABI = 12;
const FRAME_BYTES = 168;
const STACK_BYTES = 176; // Preserve the WASM stack's 16-byte alignment.
const FRAME_MAGIC = 0x5253574a;
const ERROR_OFFSET = 24;

function unsigned(value) {
  const bytes = [];
  do {
    const byte = value & 127;
    value >>>= 7;
    bytes.push(byte | (value ? 128 : 0));
  } while (value);
  return bytes;
}

function name(value) {
  const bytes = [...new TextEncoder().encode(value)];
  return [...unsigned(bytes.length), ...bytes];
}

function section(id, bytes) {
  return [id, ...unsigned(bytes.length), ...bytes];
}

// A real secondary module calls the primary helper directly. Its independently
// declared signatures also detect an incompatible raw capability at link time.
export function helperModule() {
  const body = [0, ...Array.from({ length: 10 }, (_, i) => [0x20, i]).flat(), 0x10, 0, 0x0b];
  return Uint8Array.from([
    0, 97, 115, 109, 1, 0, 0, 0,
    ...section(1, [2, 0x60, 10, 0x7f, 0x7f, 0x7f, 0x7f, 0x7e,
      0x7c, 0x7c, 0x7c, 0x7c, 0x7c, 1, 0x7c,
      0x60, 3, 0x7f, 0x7c, 0x7c, 1, 0x7c]),
    ...section(2, [2,
      ...name("rspice_jit"), ...name("eval_op_v1"), 0, 0,
      ...name("rspice_jit"), ...name("math2_v1"), 0, 1]),
    ...section(3, [1, 0]),
    ...section(7, [2, ...name("helper"), 0, 2, ...name("math2"), 0, 1]),
    ...section(10, [1, ...unsigned(body.length), ...body]),
  ]);
}

export async function qualifyAbi(wasm) {
  for (const exportName of ["__wbindgen_add_to_stack_pointer",
    "rspice_ui_wasm_jit_eval_op_v1", "rspice_ui_wasm_jit_math2_v1"]) {
    if (typeof wasm[exportName] !== "function") throw new Error(`Missing ${exportName}.`);
  }
  if (!(wasm.memory instanceof WebAssembly.Memory)) throw new Error("Missing worker memory.");
  const { instance } = await WebAssembly.instantiate(helperModule(), {
    rspice_jit: {
      eval_op_v1: wasm.rspice_ui_wasm_jit_eval_op_v1,
      math2_v1: wasm.rspice_ui_wasm_jit_math2_v1,
    },
  });
  // Reserve scratch space through the same stack export used by the pinned
  // wasm-bindgen 0.2.126 glue. Heap allocator export names are not stable.
  // All work below is synchronous; restore the stack before yielding again.
  const frame = wasm.__wbindgen_add_to_stack_pointer(-STACK_BYTES) >>> 0;
  let checks = 0;
  const view = () => new DataView(wasm.memory.buffer, frame, FRAME_BYTES);
  function reset() {
    new Uint8Array(wasm.memory.buffer, frame, FRAME_BYTES).fill(0);
    view().setUint32(0, FRAME_MAGIC, true);
    view().setUint32(4, ABI, true);
    view().setUint32(8, FRAME_BYTES, true);
  }
  function invoke(opcode, operands) {
    return instance.exports.helper(frame, opcode, 0, 0, 0n,
      ...Array.from({ length: 5 }, (_, i) => operands[i] ?? 0));
  }
  function expect(label, actual, expected, status = 0) {
    if (!Object.is(actual, expected) || view().getInt32(ERROR_OFFSET, true) !== status) {
      throw new Error(`ABI ${ABI} ${label}: value ${actual}, status ${view().getInt32(ERROR_OFFSET, true)}; expected ${expected}, ${status}.`);
    }
    checks += 1;
  }
  try {
    if (!frame || frame % 16) throw new Error("Invalid ABI fixture stack alignment.");
    for (const [label, opcode, operands, expected] of [
      // Verilog-AMS 2023 section 4.2.1.1 requires rounding to nearest.
      ["integer rounding", 300, [-7.75], -8],
      ["signed integer addition", 330, [-2147483648, 1], -2147483647],
      ["checked derivative", 340, [4, 0.25], 0.25],
      ["overflowing products", 250, [2 ** 800, 2 ** 700, 2 ** 750, 2 ** 650], 2 ** 100],
      ["underflowing products", 250, [2 ** -800, 2 ** -700, 2 ** -750, 2 ** -650], 2 ** -100],
      ["subnormal quotient", 250, [2 ** -800, 2 ** -600, 2 ** -326, 1], 2 ** -1074],
      ["signed zero quotient", 250, [-0, 1, 1, 1], -0],
    ]) {
      reset();
      expect(label, invoke(opcode, operands), expected);
    }
    reset();
    expect("hypot", instance.exports.math2(202, 3, 4), 5);
    for (const [label, opcode, operands] of [
      ["nonfinite integer", 300, [NaN]],
      ["invalid derivative primal", 340, [NaN, 0.25]],
      ["unknown operation", 249, [1]],
    ]) {
      reset();
      expect(label, invoke(opcode, operands), 0, -2);
    }
    // Reuse a previously validated allocation so the fast frame cache must
    // observe each changed header, including after a successful helper call.
    for (const [label, offset, invalid] of [
      ["stale ABI", 4, ABI - 1], ["invalid magic", 0, 0], ["short frame", 8, FRAME_BYTES - 8],
    ]) {
      reset();
      if (invoke(250, [2, 3, 1, 1]) !== 6) throw new Error("Valid frame warmup failed.");
      view().setUint32(offset, invalid, true);
      expect(label, invoke(250, [2, 3, 1, 1]), 0);
    }
    return { abiVersion: ABI, checks };
  } finally {
    wasm.__wbindgen_add_to_stack_pointer(STACK_BYTES);
  }
}

async function rejectMutation(wasm, label, acceptsError) {
  try {
    await qualifyAbi(wasm);
  } catch (error) {
    if (acceptsError(error)) return;
    throw error;
  }
  throw new Error(`The ABI gate accepted ${label}.`);
}

if (typeof self !== "undefined" && typeof self.postMessage === "function") {
  self.addEventListener("message", async ({ data }) => {
    try {
      const urls = [data.bindings, data.wasm, data.loader].map((path) => new URL(path, self.location.href));
      if (urls.some((url) => url.origin !== self.location.origin)) throw new Error("ABI fixture assets must use this origin.");
      const [module, { loadWasm }] = await Promise.all([import(urls[0].href), import(urls[2].href)]);
      const wasm = await module.default({ module_or_path: await loadWasm(urls[1]) });
      if (module.rspiceUiWasmJitAbiVersion() !== ABI) throw new Error(`The worker does not implement ABI ${ABI}.`);
      // Prove that the release harness also refuses plausible broken bindings.
      // The real exports remain immutable; only these test import maps change.
      await rejectMutation({ ...wasm, rspice_ui_wasm_jit_eval_op_v1: wasm.rspice_ui_wasm_jit_math2_v1 },
        "an incompatible helper signature", (error) => error instanceof WebAssembly.LinkError);
      await rejectMutation({ ...wasm, rspice_ui_wasm_jit_eval_op_v1: () => 0 },
        "an incorrect helper value", (error) => error.message.includes("integer rounding"));
      await rejectMutation({ ...wasm, rspice_ui_wasm_jit_eval_op_v1: (...args) => {
        const result = wasm.rspice_ui_wasm_jit_eval_op_v1(...args);
        new DataView(wasm.memory.buffer).setInt32(args[0] + ERROR_OFFSET, -2, true);
        return result;
      } }, "an incorrect helper status", (error) => error.message.includes("status -2"));
      const result = await qualifyAbi(wasm);
      self.postMessage({ status: "qualified", ...result, checks: result.checks + 3 });
    } catch (error) {
      self.postMessage({ status: "error", error: String(error) });
    }
  }, { once: true });
}
