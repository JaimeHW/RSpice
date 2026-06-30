# RSpice Xyce Regression Vendoring Notes

This directory vendors the Xyce Regression Suite runtime corpus for RSpice
multi-corpus validation work.

## Source

- Local source copied from: `C:\Users\James\Desktop\Xyce_Regression-master`
- Local runnable Xyce installation observed at:
  `C:\Users\James\Desktop\XyceNF-7.10.0\bin\Xyce.exe`
- Vendored into RSpice on: 2026-06-26
- Scope: runtime test materials only. RSpice keeps `Netlists/`, `OutputData/`,
  upstream `README.md`, `COPYING`, this note, and
  `RSPICE-HARNESS-MANIFEST.tsv`.
- Trimmed upstream harness material: `TestScripts/`, `.cir.sh` shell wrappers,
  Perl/Python helper scripts, tag/exclude selection files, and upstream
  per-directory `Manifest.txt` runner lists. RSpice discovers retained `.cir`
  decks directly and does not execute platform-specific upstream tooling.
- CTest/CMake configuration files and the CMake generator/manual artifacts are
  intentionally omitted because RSpice will not run this corpus through
  upstream CTest.

## License

The upstream `README.md` in this directory identifies Xyce as GPL-3.0-or-later
software. The local source checkout referenced a `COPYING` file, but no
`COPYING` file was present in `C:\Users\James\Desktop\Xyce_Regression-master`
at vendoring time. RSpice therefore adds `tests/xyce/COPYING` containing the
GNU GPL version 3 text from `https://www.gnu.org/licenses/gpl-3.0.txt`.

Preserve upstream copyright notices, this vendoring note, and `COPYING` when
redistributing this corpus.

## Harness Status

This corpus is not executed by `crates/rspice-core/tests/ngspice_regression.rs`.
That harness is scoped to `tests/ngspice/`. Xyce uses its own Rust-native
adapter in `crates/rspice-core/tests/xyce_regression.rs` because its
`Netlists/` and `OutputData/` layout and `.prn`-style references differ from
ngspice's checked-in `.out` convention.

`RSPICE-HARNESS-MANIFEST.tsv` records retained deck paths whose upstream source
had a `.cir.sh` wrapper sidecar. The wrapper scripts themselves are not
vendored; the manifest is the cross-platform contract the Rust adapter uses to
report those decks as expected-unsupported until wrapper semantics are
implemented natively.
