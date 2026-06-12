# 5 · Post-layout simulation

## SPEF ingestion

Reference an extractor's SPEF (IEEE 1481) file from the deck:

```spice
.spef_include "top.spef"
```

`.include top.spef` routes to the same machinery. Ingestion is true
back-annotation, applied after parsing:

- every `*I inst:pin` connection **rewires that instance terminal** from
  the ideal net onto its extracted subnode, so the wiring resistance
  genuinely sits between driver and load;
- grounded and coupling capacitances and the resistance network are
  added as ordinary elements (named `RSPEF…`/`CSPEF…`);
- `*NAME_MAP`, `*DELIMITER`, and the unit cards (`*C_UNIT`, `*R_UNIT`)
  are honored;
- ports keep the deck's node names, so external connectivity is
  untouched.

Pin names resolve against subcircuit port names for `X` instances and
standard terminal names for primitives (`D/G/S/B`, `C/B/E`, `A/K`).
A pin that cannot be matched is skipped with a warning naming it — the
run proceeds with the remaining annotation. `*INDUC` and reduced
(`*R_NET`/`*C_NET`) sections are skipped with warnings.

The log summarizes each file:

```
SPEF `top.spef`: 1243 net(s), 3104 pin(s) rewired (0 skipped), 18211 R + 22408 C added
```

## DSPF

DSPF is SPICE syntax (subcircuit-based with `*|` annotation comments) —
include it like any deck file:

```spice
.include top.dspf
```

No special handling is required; `*|`-prefixed lines are ordinary
comments to the parser.

## Practical notes

- SPEF files can be large; ingestion is linear in file size and the
  added elements use the same O(1)-stamped storage as hand-written R/C.
- Post-layout decks are solver-bound: see [chapter 8](08-performance.md)
  for the benchmark rig and solver options.
