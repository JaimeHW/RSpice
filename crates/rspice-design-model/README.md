# rspice-design-model

The durable design-management model, the drawing-sheet package contract built
on it, and the Library/Cell/View, port, bus and symbol-pin vocabulary the rest
of the design stack shares. It is data and rules over data, with no rendering
and no engine, and it depends on no other RSpice crate, so the design,
project, hardcopy, simulation and application crates share one definition
without depending on each other.

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
| `sheet_package` | The drawing-sheet preset package: build, encode and parse, with a content digest that import recomputes |

## Boundaries

- Schematic documents, connectivity, hierarchy resolution and configuration
  sets belong to [`rspice-design`](../rspice-design).
- The drawing-sheet editing surfaces and package import belong to
  [`rspice-app`](../rspice-app); printed and exported sheets are rendered by
  [`rspice-hardcopy`](../rspice-hardcopy).
- Several crates re-export these types under their own paths (the `product`
  module of `rspice-vocabulary` re-exports `ContentDigest`); the definitions
  are here.

## Entry points

- `CellViewRef::new(library, cell, view)` for a document identity.
- `DesignManagementCatalog::validate` and `semantic_digest`;
  `publish_reviewed_candidate` with the expected revision for a reviewed
  whole-catalog transaction.
- `BusDeclaration::parse` for a declared bus name.
- `custom_format(name, width_um, height_um, frame)` for a custom sheet.
- `sheet_package::build_package_with_options` and `encode_package_with_format`
  to export presets; `parse_package` to import a package.

## What the digest covers

`source_digest_sha256` is SHA-256 over the projection
`canonical_package_contract` builds: the schema, version, builtin-frame
references, source metadata and the presets. The stored digest sits outside
it. Import recomputes the digest and refuses a package that does not match.
`PACKAGE_SCHEMA` is `rspice-sheet-formats`, `PACKAGE_VERSION` is 1, and a
package is capped at `DRAWING_SHEET_PACKAGE_MAX_BYTES` (4 MiB).

## Testing

```bash
cargo test -p rspice-design-model
```

Licensed under the [RSpice Personal Use License](../../LICENSE).
