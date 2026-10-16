# rspice-design-model

The durable design-management model, the signed drawing-sheet package contract
built on it, and the Library/Cell/View, port, bus and symbol-pin vocabulary the
rest of the design stack shares. It is data and rules over data, with no
rendering and no engine, and it depends on no other RSpice crate. It stands
alone because two programs must agree on these bytes exactly: `rspice-app`
imports a signed drawing-sheet package and the offline
[`rspice-sheet-publisher`](../../tools/rspice-sheet-publisher) signs one, so
"the publisher signed it" and "the importer will accept it" are one
implementation, and the signing tool links none of the desktop application.

## What's in here

| Module | Contents |
| :--- | :--- |
| `primitives` | Leaf value types: `ContentDigest`, `ConfigurationSetId`, `Point`, `SchematicPageSize`, `SchematicPageOrientation` |
| `cell_view` | `CellViewRef`, the Library/Cell/View document identity, the name-segment rule, and the default `user` library, `top` cell and `schematic` view names |
| `port` | `PortSpec` and `PortDirection`, the interface-port identity symbols and port lists read |
| `bus` | `BusDeclaration`, `BusSlice`, the bounded bus-name parser, `declared_vector` and `declared_width` |
| `symbol_geometry` | `SymbolShape`, symbol text size, alignment and placement, and `symbol_text_bounds` |
| `symbol_pin` | `SymbolElectricalType` and `SymbolPinSide`, shared by authored and model-bound symbols |
| `design_management` | `DesignManagementCatalog`: schematic sheet catalogs and cross-sheet ports, drawing-sheet formats, presets and lettering metrics, assembly variants, reference annotation, and hierarchy audit receipts. Mutations validate a cloned candidate before committing, and each persisted structure carries its own schema version and explicit `MAX_*` bounds |
| `sheet_authoring` | `custom_format` and `validate_dimensions`: a custom drawing-sheet format derived from a `StartingFrame`, which decides margins, border and title block |
| `sheet_package` | The signed drawing-sheet package: build, encode, parse, inspect, publish and verify, with authenticity decided through a `PublisherTrust` authority |

## Boundaries

- Schematic documents, connectivity, hierarchy resolution and configuration
  sets belong to [`rspice-design`](../rspice-design).
- The revocation-aware publisher trust store the importer passes as
  `PublisherTrust` is `PdkPublisherTrustStore` in
  [`rspice-model-library`](../rspice-model-library).
- The drawing-sheet editing surfaces and package import belong to
  [`rspice-app`](../rspice-app); printed and exported sheets are rendered by
  [`rspice-hardcopy`](../rspice-hardcopy).
- Several crates re-export these types under their own paths (the `product`
  module of `rspice-app-types` re-exports `ContentDigest`); the definitions
  are here.

## Entry points

- `CellViewRef::new(library, cell, view)` for a document identity.
- `DesignManagementCatalog::validate` and `semantic_digest`;
  `publish_reviewed_candidate` with the expected revision for a reviewed
  whole-catalog transaction.
- `BusDeclaration::parse` for a declared bus name.
- `custom_format(name, width_um, height_um, frame)` for a custom sheet.
- `sheet_package::parse_package` with a `PublisherTrust` to import a package;
  `inspect_drawing_sheet_package` to read one without asserting trust.
- `publish_organization_drawing_sheet_package` and
  `verify_published_drawing_sheet_package`, the two ends of the offline
  signing ceremony.

## What a signature covers

A drawing-sheet package signature is Ed25519 over a fixed domain separator
followed by the projection `canonical_package_contract` builds. That projection
carries the schema, version, builtin-frame references, source metadata, the
publisher and signing-key identities, and the presets. The stored
`source_digest_sha256` and the signature bytes sit outside it and cannot
influence what was signed. `PACKAGE_SCHEMA` is `rspice-sheet-formats`,
`PACKAGE_VERSION` is 1, and a package is capped at
`DRAWING_SHEET_PACKAGE_MAX_BYTES` (4 MiB).

## Testing

```bash
cargo test -p rspice-design-model
```

Licensed under the [RSpice Personal Use License](../../LICENSE).
