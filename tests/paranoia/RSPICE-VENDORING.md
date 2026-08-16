# RSpice ngspice Example-Deck Vendoring Notes

This directory vendors the ngspice example decks assembled upstream for the
`paranoia` Valgrind harness, used by
`crates/rspice-conformance/src/suites/execution.rs`.

## Source

- Downloaded from <https://ngspice.sourceforge.io/tests/paranoia_parallel.7z>
  on 2026-07-28, linked from ngspice's "Tests and quality assurance" page at
  <https://ngspice.sourceforge.io/quality.html>.
- `paranoia_parallel.7z` rather than `paranoia.7z`: the two archives carry the
  same corpus, but the serial archive additionally contains a previous run's
  output artefacts (`*.out`, `*.log`, `ttt.data`, `ttt.plt`) while the
  parallel archive is clean and carries five decks the serial one lacks
  (`cider/cmosinv.cir`, `cider/pullup.cir`, `cider/recovery.cir`, and the
  `digital_devices/` group). Nothing unique to the serial archive is a deck.
- Scope: `examples/` only. RSpice discovers decks directly and does not run
  upstream's harness.

## Why this corpus, given ngspice is already vendored

`tests/ngspice/` is the ngspice *regression* suite: decks written as tests,
with checked-in `.out` references. These are ngspice's *examples* — circuits
written by people solving problems, reaching for whatever dialect corner they
needed. Upstream carries no reference output for them; RSpice therefore
maintains its own checked-in ngspice oracle tables for the simulatable decks.
They also buy breadth of ingestion. CIDER numerical devices,
`.measure` forms, transient noise, XSPICE code models, Monte Carlo,
memristors, `optran`, and PSpice-dialect vendor decks all appear here and
nowhere in the regression corpus.

## Trimmed upstream material

- Upstream harness scripts (`runtests.sh`, `paranoia_test_extra.sh`,
  `paranoia_table_generators.sh`, `textract.py`, `nggtk.tcl`, `sine.m`) and
  the Valgrind suppression file. RSpice runs these decks through its own Rust
  runner; the shell harness is neither portable to Windows nor useful here.
- Run artefacts (`*.log`).

## Excluded: third-party proprietary device models

**This is the part to preserve on any re-vendoring.** The upstream archive
redistributes vendor SPICE model libraries that carry their own copyrights
and no redistribution grant — several state "All rights reserved" outright.
They are excluded from RSpice, along with the decks that include them:

| Removed | Holder |
| --- | --- |
| `pton/` (whole directory) | ST, Infineon, Texas Instruments, Analog Devices, Microchip |
| `vdmos/` (whole directory) | Vishay; `lt-ng-mos-models-2012-2018.lib` is "Copyright (c) 2000-2012 Linear Technology Corporation" |
| `optran/models/` | Texas Instruments (OPA1611, OPA1656, TLV6001, TLV9002, TL072) |
| `vbic/Infineon_VBIC.lib` | Infineon |
| `digital/74HCng_short_2.lib` | derived from the LTspice 74HC library |
| `vbic/npn_ft.sp`, `vbic/vbic_ac_par.sp` | depend on `Infineon_VBIC.lib` |
| `digital/adder_behav.cir` | depends on `74HCng_short_2.lib` |
| `optran/HiPass3opamps_optran.cir`, `optran/TLV6001-test.cir`, `optran/TLV9002-test.cir` | depend on `optran/models/` |

Retained model files are parameter sets without a vendor copyright notice:
`optran/F5models.lib` (Toshiba 2SJ74/2SK170 parameters extracted and
published on a public forum) and `xspice/table/clc409*.sub` (a 1992
application-note macromodel carrying no rights statement).

If you re-vendor from upstream, re-run the exclusion scan rather than
trusting this table — upstream adds vendor libraries over time.

## License

The retained decks are ngspice example files, which ngspice's `COPYING`
places under the 'Modified BSD' license. The text is reproduced in
`LICENSE` beside this note. Preserve it and the upstream copyright notices
when redistributing.

## Harness status

Run by `crates/rspice-conformance/tests/execution_corpora.rs` against
`execution-manifest.tsv` in this directory. Adjacent `.oracle.out` files are
compact numerical tables captured with the official Windows console build of
ngspice 47 (`ngspice_con.exe`, SHA-256
`59225971BD68CDD1199443649AA4615A9E6D684933F205AB49006A3942518F5A`).
Ordinary tests never invoke ngspice: they execute RSpice and compare with the
checked-in table. Decks that intentionally refuse, are fragments, or remain
unsupported are recorded in the manifest with the specific reason.

Regenerate references only as an explicit maintenance action:

```powershell
cargo run --locked --release -p rspice-conformance --bin rspice-ngspice-oracle-capture -- --corpus paranoia --ngspice-exe C:\path\to\ngspice_con.exe --timeout-ms 300000 --update
```

Omit `--update` to verify that a fresh capture is byte-identical. Each file
records the ngspice version and a BLAKE3 hash of the fully expanded input.
