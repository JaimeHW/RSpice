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
  `RSPICE-HARNESS-MANIFEST.tsv` plus
  `RSPICE-UPSTREAM-EXCLUSIONS.tsv`.
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

This corpus is not executed by the ngspice regression adapter.
That harness is scoped to `tests/ngspice/`. Xyce uses its own Rust-native
adapter in `crates/rspice-conformance/tests/xyce_regression.rs` because its
`Netlists/` and `OutputData/` layout and `.prn`-style references differ from
ngspice's checked-in `.out` convention.

`RSPICE-HARNESS-MANIFEST.tsv` records retained deck paths whose upstream source
had a `.cir.sh` wrapper sidecar. The wrapper scripts themselves are not
vendored; the manifest is the cross-platform contract the Rust adapter uses to
report those decks as expected-unsupported until wrapper semantics are
implemented natively.

`RSPICE-UPSTREAM-EXCLUSIONS.tsv` is the complete, versioned provenance
manifest for all 1,143 retained decks named by the upstream exclusion files.
Its source is commit `80115a9277c0ddb3409acceb3d4e745fd11cddd4`, Netlists
tree `3e34bfaafa890cb2e4457137b6a0e325c8c1e87d`, immediately before RSpice
trimmed those platform-harness files. Each row records the original `exclude`
file even though the files themselves remain trimmed.

Upstream exclusion is the default disposition: the adapter still discovers
and counts the deck, but does not claim execution coverage from an oracle the
upstream harness excludes. This is distinct from an RSpice feature gap and is
reported separately from `expected_unsupported`.

Some upstream exclusions are helper, baseline, or control decks tested by an
upstream wrapper owner. RSpice has independently reconstructed and qualified
exact native contracts for 214 such decks. Those rows use
`rspice_independently_qualified` and name the exact expected native contract.
The adapter executes them, preserves their upstream provenance on the result,
and fails closed if execution becomes unsupported, fails, or selects a
different contract. A removed-wrapper marker by itself is not a promotion.

The Certification BUG 402 temperature-option family pins its complete
Release-7.10.0 wrapper provenance and proves that legacy `.OPTIONS TEMP=35`
and canonical `.OPTIONS DEVICE TEMP=35` produce the same 51-point DC result.
The native oracle preserves the upstream comparison direction: the Xyce
`DEVICE` spelling is GOODFILE and the SPICE-compatible unscoped spelling is
TESTFILE. It additionally validates both printed sweep coordinates against the
authored grid, closing a dormant coordinate-loop typo in the historical Perl
verifier without weakening its directional integrated-RMS value comparison.

The Certification BUG 352 model-expression family likewise reproduces its
actual Release-7.10 wrapper rather than the README's stale verifier example.
The native oracle runs the parameterized diode-model owner followed by the
literal control, proves both resolve `IS` to exactly 1.5, and compares their
case-sensitive default PRN serialization exactly as the historical shell
`diff` did. No numerical gold is invented; the excluded control is promoted
only through this paired contract.

The serial Certification BUG 307 A/B wrapper is reconstructed as an exact
subcircuit-model-scope relation. RSpice runs the collision owner followed by
the active-only control, proves that both select the qualified
`myRMOD::RMOD` card and its 0.031-ohm geometry, and compares their default PRN
serialization byte-for-byte. The separate D-I transient family keeps its
existing verifier-backed contracts and retained numerical references.

Certification BUG 267 has no numerical gold or comparator: its Release-7.10
shell wrapper only fails when the simulator process fails (the final missing-
PRN branch itself falls through with success). RSpice preserves that success
predicate and strengthens it with an exact typed contract for the retained
relative include, the `FOOBAR -> BAR` ordinary/global parameter chain, the
six-point DC grid, and the analytic voltage, source-current, and scalar-output
values. No OutputData artifact or exclusion promotion is introduced.

Where a native relational contract depends on behavior from a removed
upstream wrapper, the contract also binds the canonical historical wrapper
and verifier identities. The PARAMS1, PARSER nakedAlgebra, and Certification
BUG 1826 thermal-parameter-scope families pin the Release-7.10.0 regression
commit, their removed wrapper scripts, and the `xyce_verify.pl`
implementation that defines the fallback integrated-RMS comparison. BUG 1826
also pins the historical exclusion-list blob that selected its two executable
members. The BSRC/VCCS source-multiplicity families likewise pin the exact 20
selected owner/baseline decks, all ten removed owner wrappers, both historical
exclusion-list blobs, and the verifier. Their relational oracle preserves the
upstream direction: the authored multiplier owner is GOODFILE and the explicit
0.2-Siemens baseline is TESTFILE.

The four ABM_FREQ families also pin the Release-7.10.0 wrappers, historical
exclusion-list blob, and `ACComparator.pl`. Their native relational oracle
preserves the wrapper's directional invocation: the authored FREQ/HERTZ DEC
owner is GOODFILE, the corresponding `.AC DATA` control is TESTFILE, and the
comparison uses the exact upstream absolute, relative, zero, and frequency
tolerances.

The complete inventory and promotions are reproduced by
`tools/xyce/sync_upstream_exclusions.py`. The checked-in manifest pins the
source tree, the clean RSpice qualification commit, and the qualification
report digest; the Rust loader also pins its exact path, promotion, record,
and file identities.
