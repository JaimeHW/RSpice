# RSpice Xyce Regression Vendoring Notes

This directory vendors the Xyce Regression Suite for future RSpice
multi-corpus validation work.

## Source

- Local source copied from: `C:\Users\James\Desktop\Xyce_Regression-master`
- Local runnable Xyce installation observed at:
  `C:\Users\James\Desktop\XyceNF-7.10.0\bin\Xyce.exe`
- Vendored into RSpice on: 2026-06-26
- Scope: runtime test materials only. RSpice keeps `Netlists/`, `OutputData/`,
  `TestScripts/`, upstream `README.md`, this note, and GPL text. CTest/CMake
  configuration files and the CMake generator/manual artifacts are intentionally
  omitted because RSpice will not run this corpus through upstream CTest.

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
That harness is scoped to `tests/ngspice/`. Xyce support should be added as a
separate corpus adapter because Xyce uses `Netlists/`, `OutputData/`,
`TestScripts/`, Perl wrappers, and `.prn`-style references rather than
ngspice's checked-in `.out` convention.
