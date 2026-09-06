# rspice-publication-contract

The interchange schema for sealed RSpice circuit publications. A
`PublicationSnapshot` is the complete, self-contained description of a published
circuit page: metadata, disclosure decisions, resolved schematic scenes, the
netlist deck, analysis records, exact result datasets, measurements, and the
figures the author curated.

Three programs meet here. The RSpice client builds a snapshot at publish time,
[`rspice-publish`](../rspice-publish) renders it into an immutable page bundle,
and [`rspice-viewer`](../rspice-viewer) hydrates individual figures from
`FigurePayload`s carved out of the same types. The crate has no dependency
beyond `serde`, `serde_json`, and `thiserror`.

## Design rules

- **Typed data, never markup.** Every string is plain text that consumers escape
  at their own boundary. Nothing in a snapshot can smuggle HTML, SVG, or script
  into a rendered page.
- **Resolved geometry, not authored objects.** Schematic and plot content is a
  display list in integral micrometres (the hardcopy pipeline's convention), so
  the renderer and viewer reproduce what the author saw without compiling the
  editor's document model. Version skew between producing client and rendering
  binary cannot change what a sealed page looks like.
- **Exact numerics.** `TraceValues` carries IEEE-754 bit patterns, not decimal
  strings, so a sealed snapshot never loses source precision to formatting.
- **Strict envelopes.** Every type rejects unknown fields, roots carry an exact
  schema version, and `from_canonical_bytes` enforces a hard size cap *before*
  parsing.
- **Deterministic bytes.** No unordered collection appears in the schema;
  canonical serialization of equal values is byte-identical.

## Versions and caps

`PUBLICATION_SNAPSHOT_SCHEMA_VERSION` is 4. Versions 2 and 3 remain decodable,
their constants retained rather than deleted, because pages sealed under them
are immutable and installed clients still produce them. v3 added typed page
presentation, component/net identity, explicit signal bindings, and simulation
provenance; v4 added typed `FAILVALUE` measurement evidence. Additive evolution
happens by bumping the version, never by tolerating unrecognized content.
`FIGURE_MANIFEST_SCHEMA_VERSION` is 1 and is exact, not a floor.

Snapshots are capped at `MAX_PUBLICATION_SNAPSHOT_BYTES` (64 MiB, matching the
hardcopy worker-snapshot transport cap) and one figure payload at
`MAX_FIGURE_PAYLOAD_BYTES` (32 MiB). Inside that, thirty-odd `MAX_*` constants
bound individual fields and collection lengths: title, description,
author, label, unit, deck bytes, sheets, figures, analyses, datasets, traces per
dataset. `validate()` enforces all of them.

## Entry points

`PublicationSnapshot`, `FigureManifest`, and `FigurePayload` each expose
`canonical_bytes()` (validate, then serialize) and `from_canonical_bytes()`
(enforce the cap, parse strictly, then validate). `ContractError` names the
exact defect so a producing client's bug is reportable rather than generic.

## Testing

```text
cargo test -p rspice-publication-contract
```

Licensed under the [RSpice Personal Use License](../../LICENSE).
