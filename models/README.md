# Device models

Device model libraries bundled with the repository.

## `spice/`

SPICE model packs, organized by origin and indexed by license. See
[`spice/README.md`](spice/README.md) for the layout, the licensing tiers and the
tooling.

Four open foundry PDKs (SkyWater SKY130, GlobalFoundries GF180MCU, IHP SG13G2
and ASAP7) sit alongside academic, community and manufacturer libraries, for
roughly 191,000 model cards and 68,000 subcircuits in total. Every pack records
its upstream commit or artifact hash, its license, and whether RSpice may
redistribute it; `spice/MANIFEST.toml` is the index a packaging step reads, and
`spice/CATALOG.tsv` indexes every individual part.

`spice/builtin/` is the one pack compiled into the binary. It is the starter set
available on every platform, including the browser build, and is the only pack
whose redistribution terms must be unconditionally clear.

## `veriloga/`

Shipped Verilog-A sources for the `rspice-veriloga` compiler crate.
`constants.vams` and `disciplines.vams` provide the standard support
headers used by compact models.

`veriloga/cmc/` contains redistributable Compact Model Coalition model
packages using their upstream package directory names. Each package keeps
its upstream source layout and bundled license/notice files. The root
[`NOTICE`](../NOTICE) summarizes third-party attribution; package-specific
license files remain authoritative.

These CMC sources are not a staging area for hand-written native ports. They
are the canonical inputs for the Verilog-A to Rust generator path in
`rspice-veriloga` / `rspice-core`. Generated Rust devices should preserve the
upstream source package identity and license/notice attribution, and generated
CMC entries remain feature-gated qualification artifacts until their oracle
coverage and product gates are explicit.

OMI packages are intentionally not shipped in `veriloga/cmc/` because they are
not normal Verilog-A model packages and their terms require separate review.

Auto-discovery treats each module-bearing `.va` file under
`veriloga/<pack>/<package>/` as a candidate model source, derives package
identity from the first two path components, and scopes include directories to
the package.

RSpice does not bundle a root-level BSIM4 Verilog-A source file. Native BSIM4
v4.8 simulation is implemented in `rspice-core` for MOS `LEVEL=14/54`; a clean,
properly licensed external BSIM4 Verilog-A file can be supplied by users or
reintroduced later only with documented upstream provenance.
