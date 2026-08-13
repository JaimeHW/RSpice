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
exact native contracts for 221 such decks. Those rows use
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

The active Certification BUG 1398 wrapper compares a PSpice-style inductor
model owner with its literal-value control using Release-7.10 `xyce_verify`.
RSpice preserves that GOOD=control, TEST=owner direction, pins the complete
three-file retained family and historical wrapper/verifier provenance, and
proves the model `L` multipliers plus `TC1`/`TC2` at DEVICE TEMP=37 resolve to
the control's exact inductances before comparing their transient tables. No
numerical gold is invented.

Certification BUG 1040 SON keeps a zero-byte wrapper owner whose executable
workers differ only by ordinary operating-point startup versus the Xyce
`NOOP` spelling. RSpice pins the exact three-file retained family and removed
Release-7.10 wrapper/verifier provenance, validates the shared diode-capacitor
discharge circuit and its sole startup delta, then preserves the historical
GOOD=operating-point, TEST=NOOP `xyce_verify` direction. No numerical gold or
exclusion row is introduced.

Certification BUG 636 SON binds the active Release-7.10 error-exit wrapper and
its exact two ordered diagnostics for an incomplete `.TRAN` card. RSpice's
Xyce parser emits those diagnostics from the general missing-stop-time path,
and the conformance contract proves the retained line, source family, wrapper
ownership, nonzero parse result, and bounded no-hang behavior. No numerical
gold or exclusion row is invented.

Certification BUG 784 is intentionally distinguished from active Release-7.10
CTest coverage: its retained upstream `tags` file is exactly `exclude`, and
the generated CMake file contains no registration. RSpice nevertheless binds
the archived error-exit wrapper and its ordered diagnostic, then proves the
retained deck reaches the structured duplicate-subcircuit-port failure with
the exact conflicting formal, positions, actual nodes, and invocation under a
bounded no-hang contract. No numerical gold or exclusion row is invented.

Certification BUG 1162 SON preserves the Release-7.10 one-point behavior for
inconsistent linear, decade, and octave DC sweep directions. RSpice emits one
typed Xyce warning for each defective control, evaluates only the authored
100-volt start point, and compares every resulting default PRN table against
the wrapper's ordinary one-point baseline with the historical verifier. The
empty wrapper owner and all four controls are bound as one provenance-locked
family; no numerical gold is invented.

Certification BUG 271 preserves an active Release-7.10 success-only wrapper
whose deck proves that tab-prefixed prose and `*` lines remain comments around
an otherwise ordinary RLC/PULSE transient. RSpice pins the exact retained
source and removed wrapper provenance, validates the full typed circuit and
print expression, and requires a finite nontrivial native observation under a
shared deadline. The historical wrapper did not invoke a comparator, so no
numerical gold is invented.

Certification BUG 1661 preserves an active Release-7.10 self-relational
transient wrapper for implicit top-level `$G*` nodes. RSpice pins the sole
retained deck and complete five-file upstream family, validates the exact PWL
source, behavioral `V($g_1)` reference, resistive topology, transient card,
and print order, then requires `V($g_1)` and `V(1)` to compare numerically
equal after default eight-digit PRN serialization on every finite output row.
The historical wrapper ignored its nominal verifier and gold arguments, so no
OutputData artifact or numerical waveform oracle is invented.

Certification BUG 206 preserves an active Release-7.10 hierarchy error
wrapper. RSpice parses the exact malformed X instance, validates the otherwise
complete DC/subcircuit envelope, and requires a typed undefined-subcircuit
failure carrying the authored subcircuit and qualified instance identities.
The wrapper's source-location and undefined-subcircuit diagnostics remain
ordered, and no numerical gold or exclusion row is invented.

Certification BUG 1116 preserves an active Release-7.10 malformed-diode
error wrapper. RSpice retains the exact two-field diode card and reports a
typed missing-device-model error with source line, canonical device identity,
and device family. A repaired counterfactual is parsed to prove that the
surrounding model, transient analysis, and output request remain meaningful;
no numerical gold or exclusion row is introduced.

Certification ISSUE 61 preserves the active Release-7.10 regression for a
behavioral current source reading `I(B1)` when `B1` is itself a current-output
B source and therefore owns no MNA branch-current solution variable. RSpice
distinguishes that known non-branch device from an absent instance, reports a
typed owner/dependency/reason error through its public simulation interfaces,
and verifies the repaired voltage-output counterfactual builds. This is a
bounded crash-prevention error oracle; no numerical gold or exclusion row is
introduced.

The serial Certification BUG 307 A/B wrapper is reconstructed as an exact
subcircuit-model-scope relation. RSpice runs the collision owner followed by
the active-only control, proves that both select the qualified
`myRMOD::RMOD` card and its 0.031-ohm geometry, and compares their default PRN
serialization byte-for-byte. The separate D-I transient family keeps its
existing verifier-backed contracts and retained numerical references.

Certification BUG 28 SON's son3 wrapper is reconstructed as one three-way
mutual-inductor relation. RSpice runs the subcircuit-local parameter owner,
its literal-inductance control, and its global-parameter control under one
deadline; it first compares default PRN serialization exactly and then uses
the historical Release-7.10 `xyce_verify` direction as the wrapper did. The
contract separately proves that all three flatten to the same coupled
inductances, so waveform equality cannot conceal a shared resolution error.

Certification BUG 267 has no numerical gold or comparator: its Release-7.10
shell wrapper only fails when the simulator process fails (the final missing-
PRN branch itself falls through with success). RSpice preserves that success
predicate and strengthens it with an exact typed contract for the retained
relative include, the `FOOBAR -> BAR` ordinary/global parameter chain, the
six-point DC grid, and the analytic voltage, source-current, and scalar-output
values. No OutputData artifact or exclusion promotion is introduced.

The Certification BUG 302 delimiter wrapper is reconstructed as one typed
eight-member relation while its DC and transient workers retain their existing
independently-qualified numerical contracts. RSpice preserves the executable
wrapper's default-to-comma and default-to-tab line transformations and its
exact invalid-delimiter warning predicate. The retained OutputData additionally
proves the README's stronger claim that invalid delimiters fall back to the
default bytes, although those references were not read by the historical
wrapper. The empty owner has no invented numerical gold and no exclusion row is
changed.

The three `DIODE_ANALYTIC` transient wrappers retain no checked-in waveform.
RSpice instead reconstructs each Release-7.10 Perl generator on the native
adaptive output grid and applies the historical default `xyce_verify`
contract to the resulting diode voltage. Forward conduction and avalanche
breakdown preserve the wrapper's generated-GOOD/simulation-TEST direction;
the ordinary reverse-region case also preserves the wrapper's explicit retry
with GOOD and TEST exchanged. Admission is limited to the exact three
legacy-diode cards and their retained source/wrapper/generator provenance, so
it does not broaden the ordinary absolute transient diode envelope.

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
