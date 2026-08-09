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
