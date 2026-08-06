// Progressive figure hydration for published RSpice pages.
//
// The static document is complete without this module. Every step below is
// fail-closed: the wasm runtime and each figure payload are re-verified
// against the digests sealed into the document before anything is
// interpreted, and any rejection leaves the static SVG figure exactly as it
// was. The runtime itself performs the same verification again before it
// touches a canvas.

const island = document.getElementById("rspice-hydration");

function parseIsland() {
  try {
    const config = JSON.parse(island.textContent);
    return config && config.runtime && Array.isArray(config.figures) ? config : null;
  } catch {
    return null;
  }
}

async function fetchVerified(path, sha256Hex, byteLen) {
  const response = await fetch(path);
  if (!response.ok) {
    throw new Error(`${path}: HTTP ${response.status}`);
  }
  const bytes = await response.arrayBuffer();
  if (bytes.byteLength !== byteLen) {
    throw new Error(`${path}: expected ${byteLen} bytes, received ${bytes.byteLength}`);
  }
  const digest = await crypto.subtle.digest("SHA-256", bytes);
  const hex = Array.from(new Uint8Array(digest), (b) => b.toString(16).padStart(2, "0")).join("");
  if (hex !== sha256Hex) {
    throw new Error(`${path}: digest mismatch`);
  }
  return bytes;
}

let runtimePromise = null;

function runtime(config) {
  runtimePromise ??= (async () => {
    const wasm = await fetchVerified(
      config.runtime.wasm,
      config.runtime.wasm_sha256,
      config.runtime.wasm_byte_len,
    );
    const glue = await import(new URL(config.runtime.js, document.baseURI).href);
    await glue.default({ module_or_path: wasm });
    return glue;
  })();
  return runtimePromise;
}

async function activate(config, entry, figure, canvas) {
  const glue = await runtime(config);
  const payload = await fetchVerified(
    entry.payload.path,
    entry.payload.sha256_hex,
    entry.payload.byte_len,
  );
  const svg = figure.querySelector("svg");
  canvas.hidden = false;
  if (svg) {
    svg.style.display = "none";
  }
  try {
    await glue.hydrate_figure(canvas.id, JSON.stringify(entry), new Uint8Array(payload));
  } catch (error) {
    canvas.hidden = true;
    if (svg) {
      svg.style.display = "";
    }
    throw error;
  }
}

function main() {
  if (
    !island ||
    typeof WebAssembly !== "object" ||
    !window.isSecureContext ||
    !crypto.subtle
  ) {
    return;
  }
  const config = parseIsland();
  if (!config) {
    return;
  }
  for (const entry of config.figures) {
    const figure = document.getElementById(entry.dom_id);
    const canvas = document.getElementById(`${entry.dom_id}-canvas`);
    const button = figure ? figure.querySelector("button.hydrate") : null;
    if (!figure || !canvas || !button) {
      continue;
    }
    button.hidden = false;
    button.addEventListener("click", async () => {
      button.disabled = true;
      button.textContent = "Loading interactive view…";
      try {
        await activate(config, entry, figure, canvas);
        button.hidden = true;
      } catch (error) {
        console.warn(`${entry.dom_id}: hydration rejected, static figure retained`, error);
        button.textContent = "Interactive view unavailable";
      }
    });
  }
}

main();
