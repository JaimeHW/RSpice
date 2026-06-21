# Device models

Device model libraries bundled with the repository.

## `spice/`

SPICE `.lib` model cards authored for RSpice — starter libraries of common
discrete parts and op-amps for examples and tests: `diode.lib`,
`transistor.lib`, `mosfet.lib`, `opamp.lib`.

## `veriloga/`

Verilog-A sources for the `rspice-veriloga` compiler crate. The compact
models below are vendored from their upstream sources; each file retains
its original copyright and license header. Consolidated third-party
attribution lives in the root [`NOTICE`](../NOTICE).

| Path | Model | Origin | License |
|------|-------|--------|---------|
| `ekv26_mod.va` | EKV v2.6 MOSFET | EPFL / Tiburon Design Automation | ECL-2.0 (see file header) |
| `psp103/` | PSP 103 MOSFET + JUNCAP200 junction (CMC standard) | NXP / CEA / Arizona State University | ECL-2.0 |
| `r3_cmc/` | R3_CMC three-terminal resistor (CMC standard) | Si2 Compact Model Coalition | ECL-2.0 (`LICENSE.txt` / `NOTICE.txt` in directory) |
| `constants.vams`, `disciplines.vams` | Standard Verilog-AMS support headers | Verilog-AMS LRM | — |

`psp103/releasenotesPSP103p6.txt` is the upstream release-notes file for
the vendored PSP snapshot.

RSpice does not bundle a BSIM4 Verilog-A source file. Native BSIM4 v4.8
simulation is implemented in `rspice-core` for MOS `LEVEL=14/54`; a clean,
properly licensed external BSIM4 Verilog-A file can be supplied by users or
reintroduced later only with documented upstream provenance.
