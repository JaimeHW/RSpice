# Security policy

## Supported versions

Security fixes are applied to `main` and to the most recent tagged release.
Older pre-1.0 releases are not maintained after a replacement release is
available. Deployers should retain the exact source revision and release
attestation for every installed binary so an affected build can be identified
without relying on a mutable filename.

## Reporting a vulnerability

Report suspected vulnerabilities privately through
[GitHub's security-advisory form](https://github.com/JaimeHW/RSpice/security/advisories/new).
Do not put exploit details, proprietary circuit designs, credentials, model
libraries, or customer result data in a public issue.

Useful reports include the affected RSpice version and source commit, platform,
minimal non-confidential reproduction, impact, and any known mitigations. Good
targets include parser or model-file memory-safety issues, denial of service
that bypasses configured resource limits or timeouts, unsafe FFI/JIT behavior,
cross-tenant data exposure in an embedding service, and release or update
integrity failures.

Maintainers target acknowledgment within three business days and an initial
severity assessment within seven. Remediation timing depends on exploitability
and upstream coordination; critical confirmed issues take priority over normal
release work. These are response targets, not a contractual SLA.

## Disclosure and safe harbor

Please allow time for a fix and coordinated disclosure before publishing an
exploit. Good-faith research that avoids privacy violations, data destruction,
service disruption, and access beyond what is needed to demonstrate the issue
will not be treated as malicious activity by the project.

## Dependency risk decisions

Automated release gates run RustSec, cargo-deny, dependency review, and SBOM
generation. An advisory may be accepted only when no safe compatible upgrade
exists and reachability is constrained. Every exception is recorded in
[`security/advisory-exceptions.toml`](security/advisory-exceptions.toml) with an
owner, mitigation, exit criterion, and review date. CI fails when the registry
and scanner allowlists diverge or a review expires.
