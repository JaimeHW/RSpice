# Approved baselines

Only reviewed, reproducible, passing reports belong here. Organize them by
benchmark kind and controlled-host fingerprint:

```text
macro/<host-fingerprint>/<suite-id>-v<version>.json
klu/<host-fingerprint>/<gate-version>.json
native-jit/<host-fingerprint>/<gate-version>.json
generated-stamp/<host-fingerprint>/<gate-version>.json
```

A baseline must use the current result schema, contain raw samples and complete
tool/simulator provenance, pass its correctness preflight, come from a clean
release build, and identify an immutable suite. Ordinary runs belong in
`../results/` and CI artifact storage.

No legacy report has been promoted automatically. Promotion is an engineering
approval, not a filesystem move.

## Host-independent qualification baseline

One baseline sits outside the fingerprinted layout above, because it contains
nothing a host can influence:

```text
qualification/rspice-qualification-v1.json
```

It records what a build *is* — its minimum toolchain, its optional features,
the analysis/result matrix each non-UI surface supports (rendered from
`rspice_core::execution::capability`), the oracle evidence checked in beside
the suites, and structure digests of typed result documents produced by fixed
decks — and what it *costs*, expressed only as counts: abort polls, accepted
transient points, retained compression points, result values, and artifact
bytes.

**Wall-clock and resident set size are deliberately absent.** A shared hosted
runner measures neither reproducibly, so a time budget in a per-commit gate is
either noise or is set so loose that it catches nothing. Timing baselines stay
in the fingerprinted directories above, promoted from a controlled host under
the approval this file already describes. What the qualification baseline gates
instead is the count behind each of those costs — iterations, polls and bytes —
which is reproducible on every runner and moves for the same reasons the timing
does.

Every entry under `gates` carries its own `tolerance`, and the gate fails when
a measurement drifts past it. A tolerance of zero means the metric is exact by
construction (a sweep returns the points it was asked for; a cancelled analysis
polls nothing after it stops). The rest are counts downstream of a Newton loop,
which are stable on one target and close on the others.

Regenerate with:

```bash
RSPICE_UPDATE_QUALIFICATION_BASELINE=1 \
  cargo test --locked -p rspice-core --test qualification_baseline
```

Regeneration is the same engineering approval as promotion: the new numbers are
a claim, and the diff has to be read before it is committed.
