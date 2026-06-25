# Device models

Device model libraries bundled with the repository.

## `spice/`

SPICE `.lib` model cards authored for RSpice — starter libraries of common
discrete parts and op-amps for examples and tests: `diode.lib`,
`transistor.lib`, `mosfet.lib`, `opamp.lib`.

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
are the canonical inputs for the planned Verilog-A to Rust transpiler;
generated Rust devices should preserve the upstream source package identity
and license/notice attribution.

OMI packages are intentionally not shipped in `veriloga/cmc/` because they
are not normal Verilog-A model packages and their terms require separate
review.

Auto-discovery treats each module-bearing `.va` file under
`veriloga/<pack>/<package>/` as a candidate model source, derives package
identity from the first two path components, and scopes include directories
to the package.
