# Device models

The model libraries and model-format documents bundled with the repository.

## `spice/`

SPICE model packs, with the generated pack and part indexes that runtime
discovery reads. `spice/foundation/` is the small RSpice-authored library
compiled into every build, and `spice/SHIPPING.toml` is the sole product
allowlist. Every pack in the tree is RSpice-authored; the repository carries no
third-party SPICE model libraries.

See [`spice/README.md`](spice/README.md) for the layout, the licensing tiers,
and the index and audit tooling.

## `veriloga/`

Verilog-A sources for the `rspice-veriloga` compiler.
[`constants.vams`](veriloga/constants.vams) and
[`disciplines.vams`](veriloga/disciplines.vams) are the standard support
headers compact models include.

[`veriloga/cmc/`](veriloga/cmc/) holds the redistributable Compact Model
Coalition packages under their upstream package directory names, each keeping
its upstream source layout and its bundled license and notice files.
[`veriloga/cmc/PROVENANCE.md`](veriloga/cmc/PROVENANCE.md) records where each
one came from. The root [`NOTICE`](../NOTICE) summarizes third-party
attribution; the package-specific license files remain authoritative. OMI
packages are not shipped here: they are not ordinary Verilog-A model packages
and their terms are not covered by this tree's redistribution review.

These CMC sources are the canonical inputs to the Verilog-A to Rust generator,
not a staging area for hand-written native ports. The generated artifacts are
checked in under
[`crates/rspice-veriloga-models/`](../crates/rspice-veriloga-models/), whose
`manifest.txt` records the digest of the source tree they were generated from;
each generated device compiles in behind its own `veriloga-model-*` feature.
Attribution stays with the sources here and with the root `NOTICE`.

The other directories under `veriloga/` are non-CMC Verilog-A packages.
Discovery treats every module-bearing `.va` file under `veriloga/` as a
candidate model source, takes its package identity from the first two
directories of its path (for example `cmc/PSP104.1.0_vacode`), and scopes
include directories to that package.

No root-level BSIM4 Verilog-A source is bundled. Native BSIM4 v4.8 is
implemented in `rspice-core` for MOS `LEVEL=14/54`.

## `soa/`

Safe-operating-area curve formats. [`soa/FORMAT.md`](soa/FORMAT.md) specifies
the current/voltage SOA curve tables an SOA rule accepts: the column and row
conventions, the pulse and DC row rules, and what the curve check does and does
not infer about thermal conditions.
