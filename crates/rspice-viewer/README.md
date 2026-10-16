# rspice-viewer

The optional runtime above a published page. `rspice-publish` renders every
figure as static SVG inside the page; when a reader activates one, the page
fetches the figure's sealed hydration payload and mounts this runtime on the
figure's canvas, so schematic sheets gain pan and zoom and plots gain live axes
and a cursor readout over the sealed datasets. Hydration is progressive
enhancement: any rejection (integrity, schema or transform) leaves the static
rendering in place, so the runtime never presents an approximation of sealed
results.

## What's in here

| Module | Contents |
| :--- | :--- |
| `payload` | Re-verifies the payload's byte length and SHA-256 against its manifest entry before parsing, validates it against the contract and resolves its bindings into a plot-ready model |
| `scene` | Paints a contract `Scene` with the geometry the static SVG committed to: the same dash arrays, arc flattening against a fixed chord tolerance, and role colors from `theme` |
| `plot` | Axes, grid, series and cursor readout; tick placement and engineering-notation formatting are pure functions |
| `transform` | The single owner of trace-transform semantics. It refuses ambiguity: `Identity` on a complex trace and `PhaseDegrees` of a real trace are errors |
| `theme` | Role palettes, byte-identical to the CSS custom properties `rspice-publish` pins in the page stylesheet |
| `web` | The `wasm-bindgen` boundary, compiled for `wasm32` only |

## Boundaries

- The payload and manifest types are `rspice-publication-contract`'s.
- The page, its static SVG and its stylesheet are `rspice-publish`'s. There is
  no Cargo edge in that direction: `rspice-publish` embeds this crate's built
  WebAssembly and JavaScript glue through `RSPICE_VIEWER_RUNTIME_DIR`.

## Entry points

- `hydrate_figure(canvas_id, manifest_entry_json, payload_bytes)`, the
  JavaScript export, verifies the payload and starts the runtime on the named
  canvas. Every error is returned before the runtime starts, so a rejected
  figure keeps its SVG.
- `payload::load_figure` is the same verification and resolution as a native
  call.

## Building and testing

`eframe`, `wasm-bindgen`, `wasm-bindgen-futures` and `web-sys` are `wasm32`-only
dependencies, so everything except `web` compiles and tests on the host. The
theme parity test parses `rspice_publish::PAGE_STYLES` and fails on any color
drift between the hydrated pane and the page it replaces, which is why
`rspice-publish` is a dev-dependency.

```bash
cargo test -p rspice-viewer
```

Licensed under the [RSpice Personal Use License](../../LICENSE).
