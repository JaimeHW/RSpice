# rspice-units

The physical unit a measured value carries, and the bounded expression grammar
that unit symbols are written in. It is its own crate because the engine
(`rspice-core`) attaches units to measurement results and the results layer
(`rspice-results`) persists them as evidence: both have to agree on what a
symbol means and on how it is stored, and neither can own the definition
without the other depending on it.

## What's in here

- `lib.rs` — `MeasurementUnit` (unknown, or a known symbol), `MeasurementUnits`
  (the units of a published scalar, the scalar before projection, and its
  axis), and the operations over them: validation, conversion between
  compatible symbols, products, powers, and the interval unit of an affine
  scale.
- `parse` — the private unit-expression parser. A symbol resolves to seven
  signed exponents (time, voltage, current, temperature, angle, logarithmic
  ratio, phase-noise density) plus a scale and an additive bias. Exponents are
  stored in half-unit steps, so `sqrt(Hz)` is an exact dimension rather than a
  rounded one.

The grammar accepts `*`, `/`, `^`, parentheses and `sqrt(...)` over the
parser's named units and SI prefixes. Prefixes apply only
to the physical SI symbols, never to a percentage, an angle or an affine
temperature, so `mdegC` is not a unit. Unicode input is normalized first: `µ`
and `μ` become `u`, `Ω` becomes `ohm`, `°` becomes `deg`, `²` and `³` become
exponents, `√` becomes `sqrt`, and `·` becomes `*`. `dBc/Hz` is one named unit
rather than decibels divided by a bandwidth.

Parsing is bounded in every direction a deck or a project file could push it:
the symbol length, control characters, nesting depth, exponent magnitude and
the finiteness and sign of the resulting scale are all checked, and a symbol
that fails any of them is rejected rather than truncated.

## Boundaries

- Deciding which unit a measurement carries is `rspice-core`'s
  `analysis::measure_units`, which re-exports these types rather than defining
  its own.
- Formatting a value for a reader — engineering prefixes, decimal separators,
  temperature and angle display preferences — is `rspice-app-types`'
  `quantity`, which never rewrites a stored value.

## Entry points

- `MeasurementUnit::known` — build a known unit from a symbol, rejecting
  anything the grammar does not accept. `MeasurementUnit::Unknown` is the
  explicit record that a producer could not infer a unit; it is never a
  dimensionless one.
- `MeasurementUnit::validate` and `MeasurementUnits::validate` — re-check a
  deserialized unit, because the symbol travelled as text.
- `MeasurementUnit::convert_value` — convert a value to a requested symbol.
  It refuses incompatible dimensions, an unknown source unit, and any
  conversion whose result is not finite.
- `MeasurementUnit::product` and `::power` — derive the unit of a computed
  quantity. Either returns `Unknown` rather than a wrong unit when an operand
  is unknown, carries a bias, or the result leaves the representable range.
- `MeasurementUnit::interval` — the unit of a difference on an affine scale,
  so a temperature span is kelvin rather than degrees Celsius.
- `MeasurementUnit::same_scale_as` — whether two symbols have identical
  dimensions, scale and bias, which is the test for comparing two values
  without converting either.

## Serialized form

`MeasurementUnit` is persisted inside measurement evidence, so its
representation is a format a reader must match. It is adjacently tagged with
`kind` and `symbol` and snake-case variant names: a known volt is
`{"kind": "known", "symbol": "V"}`, and an unknown unit carries only its
`kind`. `MeasurementUnits` is a plain three-field struct. Both deny unknown
fields, so a field added here is a breaking change to stored evidence and not
a silently ignored one.

## Testing

```bash
cargo test -p rspice-units
```

Licensed under the [RSpice Personal Use License](../../LICENSE).
