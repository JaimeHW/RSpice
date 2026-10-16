# rspice-app-types

The portable value vocabulary of the RSpice application: product object
identities, the hierarchical instance-path grammar, canonical content digests,
diagnostic records, typed property values and the quantity input and display
policy. The headless domain crates (`rspice-design`, `rspice-model-library`,
`rspice-results`, `rspice-project`, `rspice-simulation`, `rspice-hardcopy`,
`rspice-formats` and their contract crates) and the egui crates
(`rspice-schematic-editor`, `rspice-results-ui`, and `rspice-ui-kit` through
its `plot` feature) all speak these values, and `rspice-app` sits above every
one of them, so the vocabulary lives below all of them in its own crate.
`tools/ci/check_app_crate_dependencies.py` keeps it free of any application
crate, egui or other GUI package, and simulator package.

## What's in here

| Module | Contents |
| :--- | :--- |
| `product` | `ProductObjectKind`, whose `ALL` lists the objects the application owns, each with a static contract (stable serialized ID, parent, owning workflow, mutability); the UUID-backed identities (`ProjectId`, `RunId`, `DatasetId` and the rest), which refuse the nil UUID; `ObjectRef`, `ObjectRevision`, `DatasetBinding`; reproducible derived and manual-deck analysis identities; `ProcessCorner` |
| `hierarchy_path` | `InstancePath`, `InstancePathPattern` and `ProbeTarget`, the one instance-path grammar. Canonical `/X1/X2` is the only stored form; engine `x1.x2` (or `x1:x2`) and the legacy `/top/...` spelling parse to the same path, and case folds for identity |
| `canonical` | `CanonicalWriter` and `content_digest`: the versioned, domain-separated SHA-256 encoding content identities are computed with, independent of serde formats, map iteration order and host word size |
| `diagnostics` | `SourceDiagnostic` over `CanonicalDiagnosticMetadata`: a source finding with a stable UUID-v5 identity, revision, currentness, related locations and quick fixes, shared by the producers and every view that lists it; `LogSeverity` for engine and application logs |
| `property` | `PropertyValue`, `PropertyDefinition` and `PropertySheet`: typed component parameters with units, ranges, visibility conditions and validation |
| `quantity` | Engineering-notation parsing and formatting, `parse_ui_quantity` for typed UI fields, `UnitsPreferences` and the `QuantityPresentationPolicy` it resolves to, and `LayoutDatabaseUnit`. This is UI policy, separate from the deck lexer, and never rewrites stored result values |
| `raw_probe` | The `V(node)`, `V(node+, node-)`, `I(source)` and `@device[i]` probe grammar shared by authored probes and retained outputs |
| `source_revision` | `SourceRevision`, a framework-independent mutation identity that derived caches compare by pointer |
| `text_validation` | Lexical bounds for persisted parameter names, single-line expressions and bounded text |

## Boundaries

- `ContentDigest` is defined in `rspice-design-model`, because persisted
  design-management records carry it; `product` re-exports it.
- Design documents, schematic objects and connectivity belong to
  `rspice-design`.
- Command identities, platform number-locale discovery and the console and
  application-log model belong to `rspice-app`.

## Entry points

- `InstancePath::parse` and `InstancePath::parse_legacy`. `fold_key` is the
  only key a map may use; `to_engine_name` and `engine_name_for` emit the
  engine spelling. Paths are bounded by `MAX_INSTANCE_PATH_BYTES` and
  `MAX_INSTANCE_PATH_DEPTH`.
- `raw_probe::validate_raw_probe` and `raw_probe::parse_probe_target` for a
  probe expression or a single probe token.
- `ProductObjectKind::contract` for an object kind's stable ID and ownership.
- `CanonicalWriter::new(domain)`, one typed write per field, then `finish()`
  for a `ContentDigest`.
- `SourceDiagnostic::current`, then `bind_validation` once the document and
  revision it was validated against are known.
- `parse_ui_quantity` with the `QuantityPresentationPolicy` from
  `UnitsPreferences::presentation_policy`, returning SI base units.
- `PropertyDefinition::new` with its builder methods, then `validate`.

## Testing

```bash
cargo test -p rspice-app-types
```

Licensed under the [RSpice Personal Use License](../../LICENSE).
