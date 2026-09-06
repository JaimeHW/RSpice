# rspice-viewer

The optional runtime above a published page. `rspice-publish` renders every
figure as static SVG inside the page document; when a reader activates one, the
page fetches that figure's sealed hydration payload and mounts this runtime on
the figure's canvas. Schematic sheets gain pan and zoom; plots gain live axes
and a cursor readout over the sealed datasets.

Hydration is progressive enhancement. Any rejection (integrity, schema, or
transform) leaves the static rendering in place, so the runtime never presents
an approximation of sealed results.

## Modules

| Module | Responsibility |
| :--- | :--- |
| `payload` | Re-verifies the payload's byte length and SHA-256 against its manifest entry, validates it against the contract, and resolves bindings into a plot-ready model |
| `scene` | Paints a contract `Scene` with the geometry the static SVG committed to: identical dash arrays, arc flattening against a fixed chord tolerance, role colors from `theme` |
| `plot` | Axes, grid, series, cursor readout; tick placement and engineering-notation formatting are pure functions so they test natively |
| `transform` | The single owner of trace-transform semantics, which the contract deliberately leaves to the runtime |
| `theme` | Role palettes, byte-identical to the CSS custom properties `rspice-publish` pins in the page stylesheet |
| `web` | The `wasm-bindgen` boundary: `hydrate_figure(canvas_id, manifest_entry_json, payload_bytes)` |

## Fail-closed, everywhere

The transport already promises integrity; `payload` re-verifies size and digest
anyway, because a truncated or tampered asset must not reach the parser.
`transform` refuses ambiguity rather than guessing: `Identity` on a complex
trace has no single honest reading, and `PhaseDegrees` of a real trace is a
producer defect, not a plot. `web::hydrate_figure` returns every error before
the runtime starts, so a rejected figure keeps its SVG.

## wasm-first, natively testable

`eframe`, `wasm-bindgen`, and `web-sys` are `cfg(target_arch = "wasm32")`
dependencies; everything except the `web` boundary compiles and tests on the
host. The theme parity test parses `rspice_publish::PAGE_STYLES` and fails on
any color drift between the hydrated pane and the page it replaces. That is
the only reason `rspice-publish` appears here, and only as a dev-dependency.
In the other direction there is no Cargo edge at all: `rspice-publish` embeds
this crate's *built* wasm and JS glue through `RSPICE_VIEWER_RUNTIME_DIR`.

## Testing

```text
cargo test -p rspice-viewer
```

Licensed under the [RSpice Personal Use License](../../LICENSE).
