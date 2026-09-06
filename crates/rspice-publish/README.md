# rspice-publish

Deterministic renderer from a sealed publication snapshot to an immutable page
bundle. The input is a validated
[`PublicationSnapshot`](../rspice-publication-contract); the output is a
`BTreeMap` from bundle-relative path to exact bytes. Identical snapshots always
produce byte-identical bundles; the cloud pipeline digests and seals what this
crate emits.

Nothing here consults the network, the clock, the environment, or any state
outside the snapshot. Every author-controlled string is escaped at this
boundary; the contract already rejects control characters, so escaping is the
only transformation text needs.

## The bundle

```text
index.html                the document, with figures as inline SVG
assets/page.css           page chrome
assets/page.js            page chrome
netlist.cir               present when the snapshot carries a deck
data/<dataset>.csv        one per result dataset
figure-manifest.json      figure ids, DOM ids, kinds, payload digests
figures/<id>.json         one sealed hydration payload per figure
assets/loader.js          hydration entry point, only when figures exist
assets/viewer.js          rspice-viewer glue, only when figures exist
assets/viewer.wasm        rspice-viewer runtime, only when figures exist
```

Page chrome is external rather than inlined so the serving policy can admit
styles and behavior from same-origin manifest entries instead of inline
executable text. A bundle with no figures ships no viewer runtime at all.

## Binary

```text
rspice-publish render --snapshot <file> --out <dir>
rspice-publish component-info
```

`--out` must not already exist: a bundle is written exactly once and never
amended, the same append-only discipline the cloud pipeline seals it under.
`component-info` prints the build identity and the digests of the viewer runtime
this binary embeds.

Exit statuses are distinct so the cloud executor can tell a producing client's
defect from its own: `2` rejected snapshot, `3` output directory exists, `4` no
viewer runtime embedded (checked before any input is read), `1` filesystem or
usage failure.

The runtime is staged by `build.rs` from `RSPICE_VIEWER_RUNTIME_DIR`. A build
without it renders nothing and says so, rather than emitting bundles whose
figures can never hydrate.

## Testing

```text
cargo test -p rspice-publish
```

Licensed under the [RSpice Personal Use License](../../LICENSE).
