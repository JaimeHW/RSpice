# Machine-readable contracts

- `suite-v1.schema.json` describes the JSON projection of a `suite.toml`.
- `macro-result-v2.schema.json` validates native end-to-end timing results,
  including identities, correctness evidence, raw samples, distributions, and
  regression verdicts.
- `artifact-envelope-v1.schema.json` is the common provenance envelope for
  KLU, native-JIT, generated-source, and generated-stamp reports. It validates
  common provenance and immutability metadata; each command's Rust type owns
  the evolving payload contract inside the envelope.

The Rust loader remains authoritative for TOML-specific path safety, duplicate
detection, and deck authentication. JSON schemas are checked in so CI artifact
consumers and dashboards can reject unknown or incomplete reports without
linking the benchmark crate.

Schema versions are compatibility boundaries. Additive implementation changes
must still satisfy the current schema; semantic or structural changes require a
new file and a methodology/baseline reset.
