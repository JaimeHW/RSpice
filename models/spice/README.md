# SPICE model library

RSpice's shipped SPICE model packs. Every pack in this tree is authored by the
RSpice project; the repository carries no third-party SPICE model libraries.
Verilog-A compact models live separately in [`models/veriloga/`](../veriloga/).

## Layout

```
models/spice/
  SHIPPING.toml       sole product-shipping allowlist
  SHIPPED-PACKS.tsv   generated runtime index for allowed packs
  SHIPPED-CATALOG.tsv generated runtime part index for allowed packs
  MANIFEST.toml       generated pack index of this tree
  PACKS.tsv           generated pack index in runtime TSV form
  CATALOG.tsv         generated part index of this tree
  LICENSE-AUDIT.tsv   generated per-file restriction findings
  foundation/         small RSpice-authored pack compiled into the binary
```

`foundation` holds `lib/` with authored model cards. Its generic cards are the
only SPICE pack compiled into RSpice and the only pack named by
`SHIPPING.toml`. Additional authored packs are developed and published through
the Model Hub pipeline, not committed to this application repository.

## Licensing

Every pack here is tier `own`: authored by the RSpice project and covered by
the repository LICENSE. The index machinery retains the full license vocabulary
(`permissive`, `copyleft`, `ambiguous`, `own`) because imported user content
and future hub packs carry real license diversity, and because
`redistributable` — the shipping decision — is recorded per pack rather than
inferred from the tier.

`SHIPPING.toml` is the sole product allowlist. Repository presence does not
grant shipping authority; release tooling fails closed on any pack it does not
name.

```bash
python tools/models/license_audit.py
```

scans every file under this tree for restriction language and writes
`LICENSE-AUDIT.tsv`, one row per finding. For an all-authored tree the expected
result is zero findings, and CI fails if it ever reports one. The audit exists
so that a card pasted in from an outside source cannot slip restriction terms
into the shipped library unnoticed.

## Working with the tree

```bash
python tools/models/license_audit.py
python tools/models/build_manifest.py
```

Regenerates `MANIFEST.toml`, `PACKS.tsv`, and `CATALOG.tsv`, plus the
allowlisted `SHIPPED-PACKS.tsv` and `SHIPPED-CATALOG.tsv`. All are committed,
and `--check` fails when any has drifted. This runs in CI and needs no network.
Regenerate in dependency order: the catalog's per-file `restricted` column is
joined from the audit.

Normal product discovery reads only the two shipped indexes. The full indexes
describe the whole tree for developer tooling; `SHIPPING.toml` selects the
product subset, and the shipped indexes are derived in the same render pass so
they cannot drift.

The generators write LF explicitly and `.gitattributes` pins it, so
regenerating on Windows and Linux produces identical output.

## Packaging notes

Native release packaging copies only `SHIPPING.toml`, the two shipped indexes,
and the allowlisted pack directories. Browser and mobile builds receive the
same foundation source through `rspice-core`; they do not carry a separate
pack tree. The Python source-distribution repair follows compiled
`include_str!` targets, so it adds only `foundation/lib/foundation.lib`.

## History

Until August 2026 this tree also vendored a large third-party developer corpus
(foundry PDKs, academic collections, community and manufacturer libraries,
~260k definitions). It was removed when RSpice committed to an entirely
RSpice-authored catalog: the corpus was never a product payload, most of it
carried no redistribution grant, and the authored-library program replaces the
rest. The vendoring tooling (`sources.toml`, `sync_packs.py`) was retired with
it; git history retains both.
