# rspice-publication-contract

The interchange schema for sealed RSpice circuit publications, shared so that
the three programs that meet here cannot disagree on a byte. `rspice-app`
builds a `PublicationSnapshot` at publish time,
[`rspice-publish`](../rspice-publish) renders it into an immutable page bundle,
and [`rspice-viewer`](../rspice-viewer) hydrates individual figures from
`FigurePayload`s carved out of the same types. The crate depends only on
`serde`, `serde_json` and `thiserror`.

## What's in here

Everything is in `src/lib.rs`:

- **The snapshot.** `PublicationSnapshot` is the complete, self-contained
  description of a published page: metadata, disclosure decisions, resolved
  schematic scenes, the netlist deck, analysis records, exact result datasets
  and measurements, page presentation, engineering identity and simulation
  provenance, and the figures the author curated.
- **The hydration handshake.** `FigureManifest` lists each hydratable figure
  with its DOM id and payload digest; `FigurePayload` is one figure's scene and
  the datasets its bindings read.
- **Validation.** The `Validate` trait, implemented by all three roots, and
  `ContractError`, which names the exact defect so a producing client's bug is
  reportable rather than generic.

## Entry points

- `PublicationSnapshot::canonical_bytes` validates, then serializes; equal
  values produce identical bytes.
- `PublicationSnapshot::from_canonical_bytes` enforces the size cap, parses
  strictly, then validates.
- `FigureManifest` and `FigurePayload` expose the same pair.

## Schema rules

- **Typed data, never markup.** Every string is plain text that consumers
  escape at their own boundary, so nothing in a snapshot can carry HTML, SVG or
  script into a rendered page.
- **Resolved geometry.** Schematic and plot content is a display list in
  integral micrometres, so the renderer and viewer reproduce what the author
  saw without compiling the editor's document model.
- **Exact numerics.** `TraceValues` carries IEEE-754 bit patterns, not decimal
  strings.
- **Strict envelopes.** Every type rejects unknown fields, and the schema
  evolves by bumping its version, never by tolerating unrecognized content.
- **Deterministic bytes.** No unordered collection appears in the schema.

## Versions and caps

A producer writes `PUBLICATION_SNAPSHOT_SCHEMA_VERSION`. The legacy versions
named by `PUBLICATION_SNAPSHOT_V2_SCHEMA_VERSION`,
`PUBLICATION_SNAPSHOT_V3_SCHEMA_VERSION` and
`PUBLICATION_SNAPSHOT_V4_SCHEMA_VERSION` still decode, because pages sealed
under them are immutable. A snapshot that declares a legacy version is refused
if it carries a field a later version introduced.
`FIGURE_MANIFEST_SCHEMA_VERSION` versions the manifest and its payloads
together, and both must declare it exactly.

A snapshot is capped at `MAX_PUBLICATION_SNAPSHOT_BYTES`, and a manifest or a
figure payload at `MAX_FIGURE_PAYLOAD_BYTES`, before parsing. The other `MAX_*`
constants bound individual text fields and collection lengths, and `validate`
enforces every one.

## Testing

```text
cargo test -p rspice-publication-contract
```

`tests/fixtures/` holds the byte-exact snapshot and manifest fixtures the
downstream crates build against. After a deliberate schema change, regenerate
them with:

```text
cargo test -p rspice-publication-contract --test golden regenerate_fixtures -- --ignored
```

Licensed under the [RSpice Personal Use License](../../LICENSE).
