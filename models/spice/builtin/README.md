# Built-in starter library

The only model pack compiled into the RSpice binary. Everything else under
`models/spice/` is loaded from disk; this pack is embedded via `include_str!`
so that a usable device set exists on every platform — including the browser
build, which has no filesystem to load the vendored packs from.

| File | Contents |
| --- | --- |
| `lib/diode.lib` | switching, rectifier and Schottky diodes, Zeners, LEDs |
| `lib/transistor.lib` | small-signal, switching and power BJTs |
| `lib/mosfet.lib` | small-signal and power MOSFETs |
| `lib/opamp.lib` | operational amplifier macromodels |

## Why this pack stays small

Breadth comes from the vendored packs: `models/spice/` carries roughly 191,000
model cards and 68,000 subcircuits covering essentially every discrete part an
engineer reaches for. This pack deliberately does not duplicate them.

What it must be instead is *unconditionally shippable*. It is linked into the
binary, so it cannot carry anything whose redistribution terms are unclear —
which rules out every `ambiguous`-tier pack, and that is where nearly all the
discrete-part breadth lives. So the rule for this directory is narrow:

> A card belongs here only if RSpice may distribute it under RSpice's own
> licence. In practice that means authored from published datasheet values, not
> copied from a vendored pack.

Datasheet figures are facts and extraction is our own work; a vendor's model
file is their expression of those facts and is not.

## Consumers

Three sites embed these files, and all three must agree:

- `crates/rspice-core/src/library/manager.rs` — the library manager, which
  indexes all four files for model and subcircuit lookup
- `crates/rspice-core/src/netlist/source_map.rs` — known-model names, so that
  built-in parts are not reported as unresolved references
- `crates/rspice-core/src/engine/builder/builtin_models.rs` — fallback model
  parameter resolution during circuit build

Renaming or moving a file here means updating all three.

## Fidelity

These are compact-model cards, not vendor-qualified macromodels. The MOSFET
cards in particular are first-order and documented as such in the file header:
they capture threshold, transconductance, channel-length modulation, body effect
and junction capacitance, but not series drain/source resistance, mobility
degradation, velocity saturation or self-heating. On-state resistance and
safe-operating-area behaviour of the power parts are therefore approximate.

Parameters the engine does not implement are omitted rather than carried as
decoration. Every card in this pack is expected to parse and solve; see the
built-in library tests in `crates/rspice-core`.
