# RSpice mixed-signal reference corpus: provenance

This directory is the mixed-signal reference corpus of
`MIXED_SIMULATION_PLAN.md` §7 (plan item P0.4), run by
`tools/rspice-conformance` (`suites::mixed`, binary `rspice-mixed-corpus`,
nightly test `tests/mixed_corpus.rs`). Each design directory holds its decks
(`deck/`), circuit sources (`net/`), oracle decks (`oracle/`), checked-in
oracle references (`ref/`), `design.toml`, `tolerance.toml` and the recorded
baseline `outcomes.tsv`.

Designs derived from the ngspice examples keep ngspice's 'Modified BSD'
licence; its text is in `LICENSE-ngspice` beside this note. RSpice-authored
files carry no third-party material.

## References

`ref/*.out` tables were captured from the oracle decks by
`rspice-mixed-corpus capture --oracle <O1|O5-O1|O5-O4|O2> --update` and are
verified byte for byte by the same command without `--update`. Each table
records the producing executable's version and SHA-256 and the BLAKE3 of its
oracle deck's expanded source; the integrity checks fail when a deck changes
without a recapture. A transient table keeps at most 20 001 rows of the
oracle's own grid (uniform by index, ends kept); only measures are compared.

| Oracle | Executable | SHA-256 |
|---|---|---|
| O1, O5-O1, O2 | ngspice-46 `ngspice_con.exe` (Windows console build) | `86c9ea5f645ca919e305639fa7bdb522355364c424d14e197f1ade617feb3453` |
| O5-O4 | Xyce 7.10 `Xyce.exe` | `e1f572cd2406bc04b8fbbddba85931346e6ab2b5f0750563848fcabecb10dfe1` |
| O5-O1 JUNCAP200 | ngspice-46 `lib/ngspice/juncap200.osdi` | `55972abf968b2cd58eed7adf95af4a6c0acd058196fa2bc5ae70668fced62af2` |
| O2 | ngspice-46 + `d_cosim` (`ivlng.dll`, `ivlng.vpi`) + Icarus Verilog 13.0 (`--enable-libvvp`) | recorded per table |

O2 decks name `simulation="ivlng"`; the capture binds the shim, its VPI module
(`RSPICE_NGSPICE_COSIM_LIB`, `RSPICE_NGSPICE_COSIM_VPI`) and runs under the
oracle environment that puts `libvvp` on `PATH`. A deck that needs ngspice's
own library directory (OSDI models) spells it `{ngspice_lib}`.

## C1 — sigma-delta modulator (`c01-sigma-delta`)

Upstream: ngspice-46 `examples/xspice/delta-sigma/`.

| File | Upstream | SHA-256 of upstream |
|---|---|---|
| `net/converter.cir` | `delta-sigma-1.cir` | `94ec027a819bc4a267f64fe998a8134ad1cdf6e92bc4a59ac02cfe0bb160313e` |
| `net/mod1-ct.cir` | `mod1-ct.cir` | `a0504dd047a043e7738cd3262409c95e01cec37cc1b3d7eccfa2310ca5e9bfb5` |
| `net/count-latch-dac.cir` | `count-latch-dac.cir` | `14ad8d815668c4c187508407d07e4c063615cb34f0631eee4117e17598a491e5` |

Edits:

- E1 `mod1-ct.cir`: close the `dac1` model's parenthesis.
- E2 `mod1-ct.cir`: `Rshunt2 inintn 0 100Meg` (upstream `initn`, a dangling node).
- E3 `count-latch-dac.cir`: `ajk7 … dout7 dnout7` (upstream drives `dnout8` twice).
- E4 `count-latch-dac.cir`: `.ends count10` (upstream `.ends count 10`).
- E5 `delta-sigma-1.cir`: the second, identical `.model adc_buff` is dropped.
- `delta-sigma-1.cir`'s input source `vin`, modulator instance `Xmod` and two
  `.include` lines move to the decks; `.csparam` and the `.control` block are
  replaced by one analysis card per deck. The control block's
  `save inp inm adaclout adaccout` becomes `.save` in the transient decks.

RSpice-authored: `net/mod1int.cir` (the integrator half of `mod1`),
`net/sdq.vams` (comparator, flip-flop and ±1 V DAC as one Verilog-AMS module),
`oracle/vams.o2/sdq_dig.v`.

## C2 — charge-pump PLL (`c02-pll`)

Upstream: ngspice-46 `examples/xspice/pll/`.

| File | Upstream | SHA-256 of upstream |
|---|---|---|
| `net/pll-xs.cir`, `net/pll-models.cir` | `shared-pll-xspice.cir` | `8116d484ecc11238034255a476e685914983a32a1e7036d149cb14d94b55903b` |
| `net/f-p-det-d-sub.cir` | `f-p-det-d-sub.cir` | `53b059f39ea08c4626cc793f6bffd91b3b88d125bbbedfe5588b870d45f9a62e` |
| `net/loop-filter-2.cir` | `loop-filter-2.cir` | `452089abe6e4b0667c4a31107ed78bcc292d11b8d6296c15536797b96da6f058` |
| `net/vco_sub_new.cir` | `vco_sub_new.cir` | `aa183b1749c63e2105269b301c1cd490a06522d68adbb49475dea8befb833d17` |

The top file `pll-xspice.cir` (`f909534ce43ee1814995909a52eb0968d1d665804507300065df4135a7bad760`)
contributes only its control block, replaced by the decks' cards.

Edits:

- `shared-pll-xspice.cir`: the duplicate `.model adc_vbuf` is split into
  `adc_ref` (threshold 0.5 V) and `adc_lvl` (vcc/2); `.csparam simtime` is
  dropped; the three `.include` lines move to the decks; the BSIM3 models
  `N1`/`P1` move unchanged to `net/pll-models.cir`; `iinj xlf.dra 0 dc 0 ac 1`
  is added (a unit AC current at the pump node, inert in transient).
- `loop-filter-2.cir`: `R1` and `C1` take deck parameters `r1v` (300) and
  `c1v` (10n) so the Monte Carlo deck can draw them with `agauss`.

RSpice-authored: `net/pll-mx.cir`, `net/loopf_mx.cir` (loopf without its
`dac_bridge`), `net/vco_pwl.va` (the `d_osc` table as a Verilog-A VCO),
`net/div20_pfd.v` (divide-by-20 and PFD), the O5 and O2 oracle decks. The O5
circuit drops the VCO, which does not load `cont`.

## C5 — LDO with a serially loaded trim DAC (`c05-ldo-trim`)

Upstream: ngspice-46 `examples/xspice/table/modelcards/` (BSIM4 benchmark
cards, Berkeley; not extracted from a real technology).

| File | Upstream | SHA-256 of upstream |
|---|---|---|
| `net/models-bsim4.cir` | `modelcard.nmos` + `modelcard.pmos` | `a6cf097047e490c7f1f743a9548d6f3032c2c9460d9999437adcfe3d3c10a374`, `4778577b2907f5bd8e68de133c257a843c1b363debeabf55d1ca013b4efd9d4b` |

Edits: the two cards are concatenated and both `VERSION`s set to 4.8.2.
`oracle/models-bsim4.xyce.inc` is the same file with `LEVEL = 54` (Xyce's
BSIM4; 4.8.2 is Xyce's highest version), and `oracle/ldo_body.xyce.inc` is
`net/ldo_body.inc` with the switch model in Xyce's `VSWITCH` form (Xyce has no
hysteretic `SW` model). Those two copies must follow their originals.

RSpice-authored: `net/ldo_body.inc`, `net/ldo.cir`, `net/esd.cir` (the
generated JUNCAP200; the ngspice decks use `juncap200.osdi`, the Xyce decks
`D … level=200`), `net/spi.cir`, `net/spi_trim.v`, all oracle decks. The
ngspice noise deck names `vin` as the input source because ngspice needs an AC
value on it and `vref` has none; the output spectrum compared does not depend
on the choice.

## P0.6 probes (`probes/`)

All RSpice-authored from design §4.
