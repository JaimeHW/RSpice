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

## C3 — d_cosim examples (`c03-dcosim`)

Upstream: ngspice-46 `examples/xspice/{verilator,icarus_verilog}`.

| File | Upstream | SHA-256 of upstream |
|---|---|---|
| `net/555.v` | `verilator/555.v` | `b8dbd124bfd71d722699cf557aa27ac544f4e4540b14f147139cc012c05a4ae6` |
| `net/adc.v`, `net/adc_nd.v` | `verilator/adc.v` | `bcb5902df556a686ede3bdb6c56858160ba0cd47eacd7b8b0571b600e2530e91` |
| `net/delay.v`, `net/delay_v2005.v` | `verilator/delay.v` | `1a4624c315abcdc356cf406f965d525ba6ddf058f9430835afdd3b8ec2287244` |
| `net/pwm.v` | `verilator/pwm.v` | `908762c0405d7829be2b50ceaa3f95492ccf6cc4814bd4ce8639b0a5e60f7728` |
| `net/555.shared`, `oracle/555.o2/deck.cir` | `verilator/555.shared`, `icarus_verilog/555.cir` | `0547e5cfd5b4510a3864358ccaeb628e0f7a51276f27725cb6a20f6e261885a2`, `93523ac827e7da28adb103a1422f215483f40cd52c86be5eec89f853a5ebef8c` |
| `net/adc.shared`, `oracle/adc.o2/deck.cir` | `verilator/adc.shared`, `icarus_verilog/adc.cir` | `d55c5efde9a3c73673db9f6b81188c9d73aaca14ab17301de10e80df7d4f68c4`, `90849d6e1ddc408108039de7889db6ee2064a8dd00ea50787feb17e92365eb92` |
| `net/delay.cir`, `oracle/delay.o2/deck.cir` | `icarus_verilog/delay.cir` | `4a5de34aac40ffa71dfb57f4e0e4453177375958b1ee1958e409afcc505958ee` |
| `net/pwm.cir`, `oracle/pwm.o2/deck.cir` | `icarus_verilog/pwm.cir` | `a93c823b4d48c4b4e0c3d8a06a2b2a829b8189dfa0a366968f54e529ecd92b46` |

Edits:
- `555.v`, `adc.v`, `delay.v`, `pwm.v`: none (verbatim, so they flip when P2
  lane L1 lands).
- `adc_nd.v`: `adc.v` without its `$display` line. `delay_v2005.v`:
  `out = out << 1;` for `out <<= 1;`, `endmodule` without `;`.
- `555.shared`: the d_cosim instance becomes
  `xverilog under_trigger over_threshold reset output qbar vl555`; the
  `.control` block is removed.
- `adc.shared`: the d_cosim instance becomes
  `xdut Clk Comp Start Sample Valid r5 r4 r3 r2 r1 r0 adc` plus six 1 ps
  `d_inverter`s into d5..d0 (replacing the inverted `~d5..~d0` ports);
  `.control` and `.end` removed.
- `delay.cir`, `pwm.cir`: the d_cosim instance and model become the decks'
  X-card (pwm with `cycles=100 samples=100`); `.control` removed.
- O2 decks: the upstream circuits inlined with explicit bridges where RSpice
  inserts built-in converters (adc_bridge at vcc/2; dac_bridge 0/vcc, 1 ps
  edges, behind 20 ohm), d_cosim `delay=1e-12`, adc wired through the same
  1 ps inverters, pwm built with `-Ppwm.Cycles=100 -Ppwm.Samples=100`,
  `.control` replaced by one card.

RSpice-authored: `net/adc_observe.cir` (a 10 ns `valid_d` strobe) and the
decks.

## C4 — 8-bit SAR ADC (`c04-sar8`)

RSpice-authored throughout: the register (`net/sar8.v`), the testbench and
register in one file (`net/sar8_full.v`), the Verilog-A comparator
(`net/cmp_va.va`), the capacitor DAC and switches (`net/sar_analog.cir`), the
sequencer wiring (`net/sar_ports.cir`, `net/sar_full.cir`), and every oracle
deck. `net/vin_codes.cir` and `net/vin_ramp.cir` are generated from the
formulas in their headers (mid-code inputs `(S_k + 0.5)/256 V`; two ramps
`(127 + (k - 4)/16)/256 V` and `(128 + (k - 4)/16)/256 V`, k = 0..7).

- `oracle/cmp_twin.cir` is the comparator's SPICE twin (G, R, C, B source) for
  ngspice.
- `oracle/sar_analog_held.cir` is `net/sar_analog.cir` with each switch
  replaced by the resistor its held control selects. ngspice-46 stamps a
  voltage-controlled switch in its off state in `.ac` and `.noise` even when the
  operating point has it on (checked on a one-switch circuit), so the O5-O1
  decks state the held switches as resistors with their thermal noise.
- `oracle/ports.o2/` runs `net/sar8.v` in Icarus through `d_cosim`
  (`build.txt`); its reference compares the code probe `v(code)`.

## C5 — LDO with a serially loaded trim DAC (`c05-ldo-trim`)

Upstream: ngspice-46 `examples/xspice/table/modelcards/` (BSIM4 benchmark
cards, Berkeley; not extracted from a real technology).

| File | Upstream | SHA-256 of upstream |
|---|---|---|
| `net/models-bsim4.cir` | `modelcard.nmos` + `modelcard.pmos` | `a6cf097047e490c7f1f743a9548d6f3032c2c9460d9999437adcfe3d3c10a374`, `4778577b2907f5bd8e68de133c257a843c1b363debeabf55d1ca013b4efd9d4b` |

Edits: the two cards are concatenated and both `VERSION`s set to 4.8.2.
`oracle/models-bsim4.xyce.inc` is the same file with `LEVEL = 54` (Xyce's
BSIM4) and both `VERSION`s set to 4.7: Xyce 7.10's 4.8.2 path evaluates the
cards' `MTRLMOD = 1` differently from ngspice-46 (about 44 % more drain
current), while its 4.7 path reproduces ngspice's currents to 0.1 %, and
ngspice's are the same for every version from 4.6.1 to 4.8.2 (the header of
that file has the measurements). `oracle/ldo_body.xyce.inc` is
`net/ldo_body.inc` with the switch model in Xyce's `VSWITCH` form (Xyce has no
hysteretic `SW` model). Those two copies must follow their originals.

RSpice-authored: `net/ldo_body.inc`, `net/ldo.cir`, `net/esd.cir` (the
generated JUNCAP200; the ngspice decks use `juncap200.osdi`, the Xyce decks
`D … level=200`), `net/spi.cir`, `net/spi_trim.v`, all oracle decks. The
ngspice noise deck names `vin` as the input source because ngspice needs an AC
value on it and `vref` has none; the output spectrum compared does not depend
on the choice.

## C6, C7, C8, C11 (`c06-buck`, `c07-i2c`, `c08-ladder`, `c11-sc`)

All RSpice-authored. C8's transistor rung includes
`../../c05-ldo-trim/net/models-bsim4.cir` (the BSIM4 benchmark cards recorded
under C5) in place, unmodified.

## C9 — power-on reset under a supply ramp (`c09-por`)

RSpice-authored throughout. `net/por_ctrl_p3.vams` repeats `net/por_ctrl.v`
with the supply-port connect modules and their `connectrules` (variant `p3`).

## C10 — digital-heavy (`c10-digital`)

**Licensing: the ISCAS-derived files fall under the hold of
`tests/iscas85/RSPICE-VENDORING.md`: internal validation, licensing decision
owed before release (amendment A5).** They are derived from these repository
files (SHA-256 of the committed LF content):

| Circuit | Source | SHA-256 |
|---|---|---|
| c432 | `tests/iscas85/85/c432/c432.net` | `cbecccb6fcb9388850a2c68a6674683b06e00bcb846fd241522b4fb983dc32dd` |
| c499 | `tests/iscas85/85/c499/c499.net` | `f7735dd7949f773dcc4df8fa66e6d7381757a9f2feede1c516f8d271c7852191` |
| c880 | `tests/iscas85/85/c880/c880.net` | `611cc68cc2f2a3efdf58d45c7245007a49a83f6922c8412d96c24f0e8f4d528c` |
| c1355 | `tests/iscas85/85/c1355/c1355.net` | `ac8b0355d34c1b7d22720a0020a7ee8cc1b9dadd7285a660c560a83b6e6d3df7` |
| c1908 | `tests/iscas85/85/c1908/c1908.net` | `7f7c02900b694c63d7bf15969ae06546fe95f1e459906b08cd3d5900cd5f0308` |
| c6288 | `tests/iscas85/85/c6288/c6288.net` | `352d43fd3f304230676b1f56a1bf9b590c22d9df4cc73fae76ffeac240b18786` |

Derived per circuit `<c>`: `net/<c>_xs.cir`, `net/<c>.v`, `net/<c>_pulse.cir`,
`net/<c>_vectors.cir`, `deck/<c>-{xs,vlog}.*.cir`,
`oracle/<c>-xs.tran.ngspice.cir`, `oracle/<c>-{xs,vlog}.o3/tb.v`. The
conversion script is not kept in the repository; this rule reproduces its
output:

- Primary inputs are the deck's `V<k> <node> gnd pulse(...)` sources (`Vdd`
  excluded), in deck order. Primary outputs are the nets of the deck's
  `.control` `save V(...)` line, or of its `.print tran` line when it has no
  control block (c499), in natural numeric order.
- Every `X<i> <in>... vdd gnd <out> <cell>` becomes, for `xs`, the XSPICE gate
  `a<i> [d_<in>...] d_<out> <model>` (a scalar input for `not1`/`buf1`), models
  `d_inverter d_buffer d_nand d_and d_nor d_or d_xor` for
  `not1 buf1 nand2-4 and2-4 nor2-4 or2-4 xor2`, rise and fall delay 10 ps;
  the inputs enter through one `adc_bridge` (0.5 V, 1 ps) onto `d_<name>` and the
  outputs leave through one `dac_bridge` (0 / 1 V) onto analog nodes of their
  own name. For `vlog` it becomes the Verilog primitive
  `<not|buf|nand|and|nor|or|xor> u<i>(<out>, <in>...);` in
  `module <c>(<inputs>, <outputs>)` (zero delay), instantiated as `xdut` with
  built-in converters at `.param vcc=1.0`.
- `net/<c>_pulse.cir` is the deck's input sources with ground spelled `0`.
  `net/<c>_vectors.cir` holds 64 vectors (c6288: 8) from Python
  `random.Random(1).getrandbits(1)`, drawn vector-major in input order, one per
  10 ns with 10 ps transitions.
- `oracle/<c>-xs.tran.ngspice.cir` includes the transistor deck verbatim by
  relative path and adds the RSpice decks' strobe and `adc_bridge` probes at
  0.9 V on the primary outputs; its control script `alter`s every input pulse
  to swing 0 -> 1.8 V with the deck's timing and runs that (the deck's own
  0 -> 1 V inputs do not resolve every 1.8 V gate: c880's all-high response
  differs). Its O1 reference is a fresh ngspice-46 capture, not the ngspice-47
  `<c>.oracle.out` beside the deck, which the mixed harness cannot read (format
  1, bound to the `.net` file); with 1 V inputs the ngspice-46 run reproduces
  every `.oracle.out` logic level at both strobes.
- The O3 templates replay the inputs and strobe of the RSpice run into
  Icarus on `net/<c>.v`; for `vlog` a wrapper instance `xdut` carries the
  converter endpoints' names.

RSpice-authored: `net/lfsr16.v`, `net/cnt16_xs.cir`, `net/r2r16.cir`,
`net/strobe_iscas.cir`, `net/strobe_vectors.cir`.

## C12 — Verilog-AMS testbench over the C5 LDO (`c12-ldo-tb`)

RSpice-authored throughout. The LDO is C5's `net/ldo_body.inc` wrapped as a
subcircuit (`net/ldo_sub.cir`, read by relative path, not copied) with C5's
BSIM4 cards (`../c05-ldo-trim/net/models-bsim4.cir`, see C5) and C5's
`net/spi_trim.v`. `net/supply_load.cir` is C5's supply and static load
without its load-step source. `net/top.vams` repeats `spi_trim.v` and
`net/tb_ldo_unrolled.vams` verbatim beside the top module, because a design
split across HDL files is plan P2 lane L8. `net/tb_ldo_unrolled.vams` and the
sixteen `oracle/c<k>.op.o5.ngspice.cir` decks are generated from the rules in
their headers. C5's ESD junction is not part of C12.

## P0.6 probes (`probes/`)

All RSpice-authored from design §4.
