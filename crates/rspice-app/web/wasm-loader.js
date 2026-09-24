// The client owns module loading in both source-tree and immutable releases.
// Release assembly compresses bytes; it does not rewrite executable source.
export async function loadWasm(moduleUrl) {
  if (!moduleUrl.pathname.endsWith(".wasm.gz")) {
    return moduleUrl;
  }
  const response = await fetch(moduleUrl);
  if (!response.ok || !response.body) {
    throw new Error(`Failed to fetch compressed RSpice module (${response.status}).`);
  }
  const stream = response.body.pipeThrough(new DecompressionStream("gzip"));
  return new Response(stream).arrayBuffer();
}
