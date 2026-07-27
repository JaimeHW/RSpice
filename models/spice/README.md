# SPICE model library

Third-party SPICE model packs vendored for RSpice, organized by origin and
indexed by license. Verilog-A compact models live separately in
[`models/veriloga/`](../veriloga/).

## Layout

```
models/spice/
  sources.toml     upstream registry - the single source of truth
  MANIFEST.toml    generated pack index, including redistribution flags
  CATALOG.tsv      generated part index, one row per .MODEL/.SUBCKT
  foundry/         open process design kits
  academic/        university and measured-silicon model sets
  community/       curated third-party collections
  vendor/          manufacturer-published libraries
```

Every pack is a directory holding:

| File | Contents |
| --- | --- |
| `pack.toml` | generated provenance, licensing and content digest |
| `LICENSE` or `LICENSE-NOTE.md` | upstream license text, or the terms as found |
| `upstream/` | upstream files, verbatim and unmodified |

Nothing under `upstream/` is edited. Fixes belong in the loader, never in the
vendored copy, so that the recorded digest keeps meaning.

## Contents

14 packs, 5,701 files, 147 MB: **191,094 model cards and 68,854 subcircuits**.

### Foundry PDKs

| Pack | Process | Devices |
| --- | --- | --- |
| `sky130` | SkyWater 130 nm CMOS | FET, diode, resistor, capacitor, bipolar, inductor, ESD |
| `gf180mcu` | GlobalFoundries 180 nm MCU | 3.3/5/6 V FET, diode, resistor, capacitor, bipolar |
| `ihp-sg13g2` | IHP 130 nm SiGe BiCMOS | SiGe HBT, MOS, Schottky, varactor, ESD |
| `asap7` | ASAP7 7 nm predictive FinFET | FinFET, SLVT/LVT/RVT/SRAM flavours |

All four carry full process-corner coverage; sky130 and IHP add mismatch and
statistical decks.

### Academic

`mosis-bsim` holds BSIM3 cards fitted to MOSIS wafer acceptance test data for
IBM 90/130/180/250 nm and TSMC 180 nm — measured silicon rather than predictive
fits, which makes it an independent cross-check for the MOS models.
`sjtu-bsim-ptm` mirrors PTM material from 180 nm to 7 nm, useful now that
`ptm.asu.edu` no longer resolves.

### Community and vendor

`microcap-library` is the broadest single source of manufacturer part models —
op-amps, regulators, power devices, logic families, optocouplers, tubes.
`ngspice-basic-models`, `ngspice-models-ugr`, the 74-series logic packs and
`ngspice-special-models` fill in curated and legacy coverage.
`diodes-inc` and `interfet-jfet` are manufacturer libraries; the InterFET file
is the reference source for discrete JFET coverage.

## Licensing

`license.tier` in each `pack.toml` governs what a build may ship. The
`redistributable` flag is what packaging reads.

| Tier | Meaning | May ship |
| --- | --- | --- |
| `permissive` | Apache-2.0, BSD, MIT, CC-BY and equivalents | yes, with a NOTICE entry |
| `copyleft` | GPL/LGPL family | not embedded in a proprietary binary |
| `ambiguous` | no explicit grant either way | decide per release |
| `own` | authored by RSpice | yes |

Currently **5 packs are permissive** (the four PDKs plus `mosis-bsim`) and
**9 are ambiguous**. Packs whose terms explicitly forbid redistribution are not
vendored at all and must not be added.

### Per-file restrictions

Pack granularity is the wrong unit for the aggregated collections. Micro-Cap's
own `about.txt` warns that *some* of its models carry a restricted commercial
licence without saying which, and the Granada collection is a similar mixture.

```bash
python tools/models/license_audit.py
```

scans every vendored file for restriction language and writes
`LICENSE-AUDIT.tsv`, one row per finding. Packaging should exclude on that
table, not on the pack.

The current result: **733 files carrying 59,810 model and subcircuit
definitions are marked `restricted`** — 23% of the tree.

| Marker | Files | What it is |
| --- | --- | --- |
| `commercial-use-restricted` | 732 | Symmetry/MODPEX generated cards. The header asserts unpublished licensed software containing proprietary information and restricts commercial use or resale under an agreement RSpice does not hold |
| `confidential` | 2 | `nation.lib` and `nichicon.LIB` are marked confidential by their originators |

The restricted set is concentrated in exactly the mainstream vendor libraries:
`On_Semi.lib` (21,923 definitions), `irf.lib` (7,333),
`Rohm_Transistor.lib` (4,713), `nation.lib` (2,785), `on_fet.lib` (2,461),
`vishaydiode.lib` (1,589). 706 of the 1,828 files in `ngspice-models-ugr` are
affected, against 27 of 181 in `microcap-library`.

A free-of-charge product tier does not clear these. The restricted axis is
commercial *use*, not sale, and a free tier inside a commercial product is still
commercial use.

The remainder is unaffected: `diodes-inc` (6,927 models), `interfet-jfet` (898),
`ngspice-basic-models`, the logic packs and the special-function models carry no
restriction marker at all. That is where the built-in library draws from, and
`tools/models/build_builtin.py` fails rather than reading a restricted file.

`diodes-inc` and `interfet-jfet` remain `ambiguous` for a different reason: they
are published for direct download with no agreement and no licence header — no
grant, and no prohibition.

## Working with the tree

```bash
python tools/models/sync_packs.py
```

Materializes every pack from its pinned upstream commit or artifact hash. Git
sources are pinned by commit, HTTP sources by sha256; a hash mismatch is a hard
error, so upstream cannot change silently under a pin.

```bash
python tools/models/build_manifest.py
```

Regenerates `MANIFEST.toml` and `CATALOG.tsv`. Both are committed, and
`--check` fails when either has drifted from the tree. This runs in CI: it
needs no network, since it only reads what is already vendored.

`sync_packs.py --check` verifies the vendored files still match their pinned
upstream. That one does reach the network, so it is a maintenance command
rather than a CI gate.

To add a pack, add one `[[pack]]` entry to `sources.toml` and run both tools.

## Known upstream defects

Some vendored files are not directly `.include`-able. This is a property of the
upstream data, not of the parser — where noted, the reference simulator fails on
them too:

- `diodes-inc`: the bulk catalog carries part-label prefixes and prose lines
  between cards. Line 2543 reads `B540C.LIB B540C`, which hides the `.LIB` from
  the parser and orphans the `.ENDL` at line 2552. ngspice-46 also fails on this
  file, at line 3260. It is a catalog to extract cards from, not a library to
  include.
- `ihp-sg13g2`: diode subcircuits pass arithmetic in instance parameters
  (`area=mf*aws`), which RSpice does not yet evaluate at diode instance sites.
