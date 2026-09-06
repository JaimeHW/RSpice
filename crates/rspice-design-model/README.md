# rspice-design-model

The durable design-management model and the signed drawing-sheet package
contract built on top of it. Everything here is data and rules over data:
identities, revisions, semantic digests, sheet geometry, and the canonical
package a publisher signs. No rendering, no widgets, no engine.

The crate exists because two very different programs must agree on these bytes
exactly. [`rspice-ui`](../rspice-ui) imports a signed drawing-sheet package; the
offline [`rspice-sheet-publisher`](../rspice-sheet-publisher) ceremony signs
one. Sharing one implementation is what makes "the publisher signed it" and "the
importer will accept it" the same statement, and it keeps the publisher's
trusted computing base down to `serde`, SHA-256, and Ed25519 rather than the
whole desktop application. `rspice-ui` re-exports these types from the modules
that used to define them, so application paths are unchanged.

## Modules

- **`primitives`**: leaf value types with no behaviour beyond their own
  invariants: `ContentDigest`, `ConfigurationSetId`, `Point`,
  `SchematicPageSize`, `SchematicPageOrientation`.
- **`design_management`**: the authority over schematic sheets, assembly
  variants, reference annotation, and hierarchy preflight evidence. Split into
  `identity`, `drawing_sheet`, `sheets`, `variants`, `annotation`, `hierarchy`,
  and `catalog`.
- **`sheet_authoring`**: deriving a custom drawing-sheet format from a
  `StartingFrame`, because a custom size is not just a width and a height: the
  drafting convention it starts from decides margins, border, and title block.
- **`sheet_package`**: the signed package contract.

## Draft in, committed state out

The UI only edits drafts. Every mutation in `design_management` is applied to a
cloned candidate and committed after complete validation, so malformed or stale
dialog input cannot partially change a project. Stable identities, revisions,
semantic digests, and immutable receipts are owned here rather than inferred
from labels the workbench rendered.

Each persisted structure carries its own schema version
(`DESIGN_MANAGEMENT_SCHEMA_VERSION`, `SHEET_CATALOG_SCHEMA_VERSION`,
`VARIANT_CATALOG_SCHEMA_VERSION`, `ANNOTATION_STATE_SCHEMA_VERSION`) and an
explicit `MAX_*` bound on every collection and string, so a corrupted or hostile
project file is rejected rather than allocated against.

## What a signature covers

A drawing-sheet package signature covers exactly the projection built by
`canonical_package_contract`, prefixed with a domain separator. The stored
digest and the signature bytes themselves sit *outside* that projection and so
cannot influence what was signed. `PACKAGE_SCHEMA` is `rspice-sheet-formats`,
`PACKAGE_VERSION` is 1, and a package is capped at
`DRAWING_SHEET_PACKAGE_MAX_BYTES` (4 MiB).

`publish_organization_drawing_sheet_package` and
`verify_published_drawing_sheet_package` are the two ends of the ceremony;
`inspect_drawing_sheet_package` reads one without asserting trust.

## Testing

```text
cargo test -p rspice-design-model
```

Licensed under the [RSpice Personal Use License](../../LICENSE).
