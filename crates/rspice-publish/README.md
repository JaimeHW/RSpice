# rspice-publish

The deterministic renderer from a sealed publication snapshot to an immutable
page bundle. The input is a validated `PublicationSnapshot` from
[`rspice-publication-contract`](../rspice-publication-contract); the output is a
`Bundle`, a `BTreeMap` from bundle-relative path to exact bytes. Identical
snapshots always produce byte-identical bundles; the cloud pipeline digests
and seals what this crate emits. Nothing here consults the network, the clock,
the environment, or any state outside the snapshot, and every
author-controlled string is escaped at this boundary.

## What's in here

- `src/lib.rs`: `render_bundle`, the bundle layout, dataset CSV, and HTML
  escaping.
- `page`: the `index.html` document and its external chrome
  (`src/assets/page.css`, `src/assets/page.js`).
- `svg`: scenes rendered as inline SVG.
- `hydration`: `ViewerRuntime`, the page loader (`src/assets/loader.js`), and
  the digests it verifies before hydrating a figure.
- `src/main.rs` and `build.rs`: the `rspice-publish` binary, which embeds the
  viewer runtime at build time.

## Boundaries

- The snapshot schema, its caps and its validation:
  `rspice-publication-contract`.
- The figure runtime a bundle ships: [`rspice-viewer`](../rspice-viewer).
- Building a snapshot from a project: `rspice-app`.

## Entry points

- `render_bundle(snapshot, snapshot_sha256_hex, viewer)` renders one bundle;
  the digest is that of the canonical snapshot bytes, printed in the page's
  provenance footer.
- `ViewerRuntime::new(wasm, js_glue)` accepts the built viewer runtime.
- `escape_html` is the escaping every author-controlled string passes through.

## The bundle

```text
index.html                     the document, with figures as inline SVG
assets/page.css                page chrome
assets/page.js                 page chrome
netlist.cir                    present when the snapshot carries a deck
data/<id>-<name>[-<variant>].csv  one per result dataset
figure-manifest.json           figure ids, DOM ids, kinds, payload digests
figures/<id>.json              one hydration payload per figure
assets/loader.js               hydration entry point, only when figures exist
assets/viewer.js               rspice-viewer glue, only when figures exist
assets/viewer.wasm             rspice-viewer runtime, only when figures exist
```

Page chrome is external rather than inline so the serving policy can admit
styles and behaviour from same-origin bundle entries instead of inline
executable text. A bundle with no figures ships no viewer runtime.

## Binary

```text
rspice-publish render --snapshot <file> --out <dir>
rspice-publish component-info
```

`--out` must not already exist: a bundle is written exactly once and never
amended. `component-info` prints the version and whether a viewer runtime is
embedded, with the runtime's digests and sizes when it is.

`render` exits with a status the cloud executor uses to tell a producing
client's defect from its own:

| Status | Cause |
| --- | --- |
| `0` | Bundle written |
| `1` | Filesystem failure, or a usage error |
| `2` | Snapshot rejected by the contract |
| `3` | Output directory already exists |
| `4` | No viewer runtime embedded, checked before any input is read |

## Building and testing

`build.rs` embeds the runtime it finds in `RSPICE_VIEWER_RUNTIME_DIR`, which
must hold `viewer_bg.wasm` and `viewer.js` as `wasm-bindgen` produced them from
`rspice-viewer`. A build without the variable compiles, but its binary refuses
to render; the library tests supply their own runtime bytes.

```text
cargo build --locked --profile viewer-release -p rspice-viewer \
  --target wasm32-unknown-unknown
wasm-bindgen --target web --out-name viewer --out-dir <dir> \
  target/wasm32-unknown-unknown/viewer-release/rspice_viewer.wasm
RSPICE_VIEWER_RUNTIME_DIR=<dir> cargo build --locked --release -p rspice-publish
cargo test -p rspice-publish
```

Licensed under the [RSpice Personal Use License](../../LICENSE).
