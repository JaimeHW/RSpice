import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { gzipSync } from "node:zlib";
import test from "node:test";

const source = await readFile(new URL("../../crates/rspice-ui/web/wasm-loader.js", import.meta.url));
const { loadWasm } = await import(`data:text/javascript;base64,${source.toString("base64")}`);
const wasm = Uint8Array.from([0, 97, 115, 109, 1, 0, 0, 0]);

test("development URLs reach wasm-bindgen without an eager fetch", async (t) => {
  const fetch = t.mock.method(globalThis, "fetch", () => assert.fail("unexpected fetch"));
  const url = new URL("https://example.test/pkg/rspice-ui_bg.wasm?v=name.wasm.gz");
  assert.equal(await loadWasm(url), url);
  assert.equal(fetch.mock.calls.length, 0);
});

test("both release modules inflate exact Wasm bytes", async (t) => {
  const requested = [];
  t.mock.method(globalThis, "fetch", async (url) => {
    requested.push(url.href);
    return new Response(gzipSync(wasm));
  });
  for (const module of ["rspice-ui", "rspice-ui-worker"]) {
    const url = new URL(`https://example.test/ide/assets/${"a".repeat(64)}/${module}_bg.wasm.gz`);
    const bytes = await loadWasm(url);
    assert.deepEqual(new Uint8Array(bytes), wasm);
    assert.equal(WebAssembly.validate(bytes), true);
    assert.equal(requested.at(-1), url.href);
  }
});

test("missing and bodyless responses fail startup", async (t) => {
  const url = new URL("https://example.test/module.wasm.gz");
  const responses = [new Response("missing", { status: 404 }), new Response(null)];
  t.mock.method(globalThis, "fetch", async () => responses.shift());
  await assert.rejects(loadWasm(url), /compressed RSpice module \(404\)/);
  await assert.rejects(loadWasm(url), /compressed RSpice module \(200\)/);
});

test("truncated compressed bytes cannot become a loaded module", async (t) => {
  const compressed = gzipSync(wasm);
  t.mock.method(globalThis, "fetch", async () => new Response(compressed.subarray(0, 12)));
  await assert.rejects(loadWasm(new URL("https://example.test/module.wasm.gz")));
});
