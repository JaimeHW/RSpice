# Built-in starter library

The only model pack compiled into the RSpice binary. Everything else under
`models/spice/` is loaded from disk; this pack is embedded via `include_str!`
so a usable device set exists on every platform — including the browser build,
which has no filesystem to load the vendored packs from.

| File | Contents |
| --- | --- |
| `lib/diode.lib` | switching, rectifier and Schottky diodes, zener, power LED |
| `lib/bjt.lib` | small-signal, switching and power bipolars |
| `lib/jfet.lib` | N- and P-channel junction FETs |
| `lib/mosfet.lib` | small-signal MOSFETs and power VDMOS parts |
| `lib/opamp.lib` | operational amplifier macromodels |
| `lib/ic.lib` | 555 timer and linear voltage regulators |

36 cards, roughly 42 KB.

## Nothing here is hand-authored

`lib/` is **generated**. `selection.toml` names cards that already exist in the
vendored packs, and `tools/models/build_builtin.py` extracts them:

```bash
python tools/models/build_builtin.py
```

Model parameters are copied byte for byte. A selection entry may set `name` to
rename a card to the part's canonical designation — `DI_1N4001` to `1N4001`,
`J310-LS` to `J310` — which rewrites one token on the header line and nothing
else. Every card carries a comment giving its source pack, file and line.

To add a part: find it in `models/spice/CATALOG.tsv`, add an entry to
`selection.toml`, regenerate, and run the tests.

Two granularities are available, and the distinction matters:

- `[[file]]` copies a whole source file. This is the correct unit for
  subcircuits, because a macromodel depends on internal `.model` and `.subckt`
  definitions that live alongside it. Extracting the subcircuit alone would
  strip them.
- `[[model]]` extracts a single named `.model` card with its continuation
  lines, for pulling one part out of a large flat catalog.

## Licensing

The generator refuses to read any file that `tools/models/license_audit.py`
marks `restricted`, and fails the build rather than skipping it. Those files are
excluded from a distributed build, so a card lifted out of one would smuggle the
restriction into the binary where it cannot be stripped later.

That check is not theoretical. The mainstream vendor libraries in the packs —
`On_Semi.lib`, `irf.lib`, `Rohm_Transistor.lib` — are Symmetry/MODPEX generated
and carry an explicit commercial-use restriction. The power MOSFETs here come
from ngspice's VDMOS cards instead, which are both unrestricted and a better fit
for the engine.

## Fidelity

These are real manufacturer and databook models, not simplified stand-ins:

- **BC546B / BC556B** — Philips SC04 databook Gummel-Poon parameters.
- **1N4148** — extracted from measured data, valid −55 to 125 °C.
- **IRFP240 / IRFP9240** — ngspice VDMOS cards with Rd/Rs/Rg, body diode and a
  thermal network, so on-resistance and conduction loss are actually modelled.
- **TL072 / TL082 / OP07 / LM358 / MCP6041** — vendor macromodels with real
  slew limiting, output clamping and supply current.

A predecessor of this pack was a set of hand-written LEVEL=1 cards named after
real parts. A LEVEL=1 die called `IRF540` has no series resistance, so its
on-resistance is zero and every switching result is silently wrong. That is why
selection now comes from the packs, and why the tests below exist.

## Tests

`crates/rspice-core/tests/builtin_library.rs` walks the embedded content and
requires every probeable card to parse and solve, so parts added to
`selection.toml` are covered automatically. It also asserts that the power
MOSFET cards show a real conduction drop.

## Consumers

Three sites embed these files, and all three must agree:

- `crates/rspice-core/src/library/manager.rs` — indexes all six files for model
  and subcircuit lookup
- `crates/rspice-core/src/netlist/source_map.rs` — known-model names, so
  built-in parts are not reported as unresolved references
- `crates/rspice-core/src/engine/builder/builtin_models.rs` — fallback model
  parameter resolution during circuit build

Adding or renaming a category file means updating all three.
