# rspice-cloud-domain

The vocabulary RSpice Cloud is written in: identifiers, policy enumerations,
execution provenance, and the two versioned hashing contracts that decide when
two things are the same thing.

It is the base of the cloud dependency chain (`rspice-cloud-contract` and
`rspice-cloud-client` sit above it) and it deliberately has no database, HTTP,
async runtime, or platform dependency. `#![forbid(unsafe_code)]`.

## What is here

| Module | Contents |
| :--- | :--- |
| `ids` | `WorkspaceId`, `CircuitId`, `RevisionId`, `SimulationRunId`, `ShareId`, `PrincipalId`: distinct newtypes so a run id cannot be passed where a circuit id belongs |
| `policy` | `WorkspaceRole`, `SharePermission`, `CircuitVisibility`, `EntitlementStatus`, `SimulationRunStatus` |
| `revision` | `revision_content_digest`, the digest that gives an immutable circuit revision its identity |
| `simulation` | `simulation_request_digest`, the digest that makes a resolved remote-simulation request cacheable and idempotent |
| `execution` | `SimulationExecutionManifest` and its artifact, engine, and runtime-mode records: the customer-visible provenance of one run |

## Versioned digests

Both digest functions are stable hashing contracts, not conveniences. Changing
what bytes feed them changes which historical revisions and runs are considered
identical, so each carries an explicit version constant and the previous version
is retained rather than deleted:

- `CURRENT_REVISION_CONTENT_DIGEST_VERSION` /
  `LEGACY_REVISION_CONTENT_DIGEST_VERSION`
- `CURRENT_SIMULATION_REQUEST_DIGEST_VERSION`
- `CURRENT_SIMULATION_EXECUTION_MANIFEST_VERSION`,
  `LEGACY_SIMULATION_EXECUTION_MANIFEST_VERSION`, and
  `VERIFIED_ADAPTER_SIMULATION_EXECUTION_MANIFEST_VERSION`, which marks a
  manifest produced by the credentialless `rspice-engine-adapter` path

Execution manifests are also bounded before they are believed:
`MAX_SIMULATION_EXECUTION_MANIFEST_BYTES`,
`MAX_SIMULATION_EXECUTION_ARTIFACTS`,
`MAX_SIMULATION_EXECUTION_ARTIFACT_BYTES`, and `MAX_SIMULATION_ATTEMPTS` are
checked by `is_valid_simulation_execution_manifest`.

## Testing

```text
cargo test -p rspice-cloud-domain
```

Licensed under the [RSpice Personal Use License](../../LICENSE).
